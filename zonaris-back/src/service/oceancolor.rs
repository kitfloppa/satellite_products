use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use axum::async_trait;
use chrono::prelude::*;
use image::ImageBuffer;
use log::{error, info};
use reqwest::redirect::{DefaultFilter, Filter};
use tokio::sync::Mutex;
use tokio_cron_scheduler::Job;

use crate::{
    persistence::{
        model::{instrument_data::InstrumentData, oceancolor::OceanColorMapping},
        Repository,
    },
    utils::geophysical_data::GeophysicalData,
};

use super::InstrumentDataService;

pub struct SearchItem(String);

impl SearchItem {
    fn new(name: String) -> SearchItem {
        SearchItem(name)
    }

    pub fn get_time(&self) -> Result<NaiveDateTime, String> {
        return NaiveDateTime::parse_from_str(
            self.0.split('.').nth(1).ok_or("bad format")?,
            "%Y%m%dT%H%M%S",
        )
        .map_err(|r| r.to_string());
    }
}

#[async_trait]
pub trait OceanColorService {
    async fn search(
        &self,
        sdate: NaiveDateTime,
        edate: NaiveDateTime,
        mapping: &OceanColorMapping,
    ) -> Result<Vec<SearchItem>>;

    async fn get(&self, item: SearchItem) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>>;
}

pub struct OceanColorServiceDefault {
    oceancolor_authorization: String,
    oceancolor_mapping_repository: Repository<OceanColorMapping>,
    instrument_data_service: InstrumentDataService,
}

struct JobState {
    last_date: Option<NaiveDateTime>,
}

impl JobState {
    fn new() -> JobState {
        return JobState { last_date: None };
    }
}

pub struct OceanColorJobSettings {
    pub time_step: std::time::Duration,
    pub not_found_duration: chrono::Duration,
}

impl OceanColorServiceDefault {
    pub fn new(
        oceancolor_authorization: String,
        oceancolor_mapping_repository: Repository<OceanColorMapping>,
        instrument_data_service: InstrumentDataService,
    ) -> OceanColorServiceDefault {
        return OceanColorServiceDefault {
            oceancolor_authorization,
            oceancolor_mapping_repository,
            instrument_data_service,
        };
    }

    async fn job_func(
        oceancolor_service: Arc<OceanColorServiceDefault>,
        job_state: Arc<Mutex<JobState>>,
        settings: Arc<OceanColorJobSettings>,
    ) -> Result<()> {
        let edate = Utc::now().naive_utc();

        let sdate = {
            let mut job_state = job_state.lock().await;

            let r = if let Some(last_date) = job_state.last_date {
                last_date
            } else {
                Utc::now()
                    .checked_sub_signed(settings.not_found_duration)
                    .unwrap()
                    .naive_utc()
            };

            job_state.last_date = Some(edate);

            r
        };

        let mappings = {
            let lock = oceancolor_service
                .oceancolor_mapping_repository
                .read()
                .await;

            lock.get_all().await?
        };

        for mapping in mappings {
            let items = oceancolor_service.search(sdate, edate, &mapping).await?;

            info!(
                "found {} items in range ({}; {})",
                items.len(),
                sdate,
                edate
            );

            let current_time = Utc::now();
            let subfolder = current_time.format("%Y%m%d").to_string();
            let fileset = current_time.format("%H%M%S").to_string();
            let base_path = format!("images/{}", subfolder);

            std::fs::create_dir_all(&base_path)?;

            for (idx, item) in items.into_iter().enumerate() {
                let img = oceancolor_service.get(item).await?;
                let img_path = format!("{}/{}_{}.png", base_path, fileset, idx);
                img.save(&img_path)?;

                let satellite_data = InstrumentData::new(*mapping.satellite_instrument_id, img_path);
                if !oceancolor_service
                    .instrument_data_service
                    .add_data(satellite_data)
                    .await?
                {
                    error!("failed to add new data");
                }
            }
        }

        return Ok(());
    }

    // TODO: i think it can be located in trait
    pub fn create_job(self: &Arc<Self>, settings: OceanColorJobSettings) -> Result<Job> {
        let oceancolor_service = self.clone();
        let job_state = Arc::new(Mutex::new(JobState::new()));
        let settings = Arc::new(settings);

        let job = Job::new_repeated_async(settings.time_step, move |_uuid, _job_scheduler| {
            let oceancolor_service = oceancolor_service.clone();
            let job_state = job_state.clone();
            let settings = settings.clone();

            return Box::pin(async move {
                if let Err(err) =
                    OceanColorServiceDefault::job_func(oceancolor_service, job_state, settings)
                        .await
                {
                    error!("{}\n{}", err, err.backtrace());
                }
            });
        })?;

        return Ok(job);
    }
}

pub struct AllowCrossOrigin<T>
where
    T: Filter,
{
    underlying_filter: T,
}

impl<T> Default for AllowCrossOrigin<T>
where
    T: Filter + Default,
{
    fn default() -> Self {
        Self {
            underlying_filter: T::default(),
        }
    }
}

const NASA_GOV: &str = "nasa.gov";

impl<T> Filter for AllowCrossOrigin<T>
where
    T: Filter,
{
    fn handle_sensitive_headers(
        &self,
        headers: &mut reqwest::header::HeaderMap,
        next: &reqwest::Url,
        previous: &[reqwest::Url],
    ) {
        let nasa_gov_len = NASA_GOV.len();

        let filter = if let Some(domain) = next.domain() {
            !(domain.ends_with(NASA_GOV)
                && (domain.len() == nasa_gov_len
                    || domain.chars().nth_back(nasa_gov_len) == Some('.')))
        } else {
            true
        };

        if filter {
            self.underlying_filter
                .handle_sensitive_headers(headers, next, previous);
        }

        return;
    }
}

#[async_trait]
impl OceanColorService for OceanColorServiceDefault {
    async fn search(
        &self,
        sdate: NaiveDateTime,
        edate: NaiveDateTime,
        mapping: &OceanColorMapping,
    ) -> Result<Vec<SearchItem>> {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let sdate = sdate.format(fmt).to_string();
        let edate = edate.format(fmt).to_string();

        let sensor_id = mapping.sensor_id.to_string();
        let dtid = mapping.data_id.to_string();

        let mut params = HashMap::new();
        params.insert("results_as_file", "1");
        params.insert("sensor_id", &sensor_id);
        params.insert("dtid", &dtid);
        params.insert("sdate", &sdate);
        params.insert("edate", &edate);
        params.insert("subType", "1");

        let response = reqwest::Client::new()
            .post("https://oceandata.sci.gsfc.nasa.gov/api/file_search")
            .form(&params)
            .send()
            .await?;

        let names = response
            .text()
            .await?
            .lines()
            .map(|line| String::from(line))
            .collect::<Vec<_>>();

        if names.len() == 1 && names[0] == "No Results Found" {
            return Ok(Vec::new());
        }

        return Ok(names
            .into_iter()
            .map(|name| SearchItem::new(name))
            .collect::<Vec<_>>());
    }

    async fn get(&self, item: SearchItem) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
        // todo migrate to tempfile
        let tmpdir = tempfile::tempdir()?;

        let cookie_provider = Arc::new(reqwest::cookie::Jar::default());

        let mut authorization_header_value = reqwest::header::HeaderValue::from_str(&format!(
            "Basic {}",
            self.oceancolor_authorization
        ))?;
        authorization_header_value.set_sensitive(true);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, authorization_header_value);

        let mut redirect_policy = reqwest::redirect::Policy::default();
        redirect_policy.set_filter(Box::new(AllowCrossOrigin::<DefaultFilter>::default()));

        let getfile_baseurl =
            reqwest::Url::from_str("https://oceandata.sci.gsfc.nasa.gov/cgi/getfile/")?;

        // CHECK THIS: https://oceancolor.gsfc.nasa.gov/data/download_methods/
        let response = reqwest::ClientBuilder::new()
            .redirect(redirect_policy)
            .default_headers(headers)
            .cookie_provider(cookie_provider.clone())
            .build()?
            .get(getfile_baseurl.join(&item.0)?)
            .send()
            .await?;

        let fname = String::from(
            response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .ok_or(anyhow!(
                    "filename extraction from url failed. url: {}",
                    response.url().clone()
                ))?,
        );

        let contents = response.bytes().await?;

        // TODO: it's can be done in RAM but netcdf library doesn't have implementation for reading from memory (https://docs.unidata.ucar.edu/netcdf-c/4.8.1/md_inmemory.html)
        let tmppath = tmpdir.path().join(&fname);
        std::fs::write(&tmppath, contents)?;
        let file = netcdf::open(&tmppath)?;

        let sst4 = GeophysicalData::load_netcdf(&file, "sst4")?;

        return Ok(sst4.generate_image());
    }
}
