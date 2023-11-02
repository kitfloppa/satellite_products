use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::async_trait;
use chrono::prelude::*;
use image::ImageBuffer;

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
    ) -> Result<Vec<SearchItem>, Box<dyn std::error::Error>>;

    async fn get(
        &self,
        item: SearchItem,
    ) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>>;
}

pub struct OceanColorServiceDefault {
    oceancolor_authorization: String,
}

impl OceanColorServiceDefault {
    pub fn new(oceancolor_authorization: String) -> OceanColorServiceDefault {
        OceanColorServiceDefault {
            oceancolor_authorization,
        }
    }
}

struct GeophysicalData {
    name: String,
    data: Vec<f32>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
enum GeophysicalDataError {
    Netcdf(netcdf::error::Error),
    InvalidFileStructure(String),
    VariableNotFound(String),
    AttributeNotFound(String),
    FilenameExtractionFromUrlFailed(reqwest::Url),
}

impl std::fmt::Display for GeophysicalDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred in the GeophysicalData module.")
    }
}

impl std::error::Error for GeophysicalDataError {}

impl GeophysicalData {
    fn get_attr<'a, T>(var: &netcdf::Variable<'a>, name: &str) -> Result<T, GeophysicalDataError>
    where
        T: TryFrom<netcdf::AttrValue, Error = netcdf::error::Error>,
    {
        return T::try_from(
            var.attribute(name)
                .ok_or(GeophysicalDataError::AttributeNotFound(String::from(name)))?
                .value()
                .map_err(|r| GeophysicalDataError::Netcdf(r))?,
        )
        .map_err(|r| GeophysicalDataError::Netcdf(r));
    }

    fn load_netcdf(
        file: &netcdf::File,
        name: &str,
    ) -> Result<GeophysicalData, GeophysicalDataError> {
        let geophysical_data = file
            .group("geophysical_data")
            .map_err(|r| GeophysicalDataError::Netcdf(r))?
            .ok_or(GeophysicalDataError::InvalidFileStructure(String::from(
                "failed to fetch 'geophysical_data' group",
            )))?;
        let sst4 = geophysical_data
            .variable(name)
            .ok_or(GeophysicalDataError::VariableNotFound(String::from(name)))?;

        let fill_value = GeophysicalData::get_attr::<i16>(&sst4, "_FillValue")?;

        let scale_factor = GeophysicalData::get_attr::<f32>(&sst4, "scale_factor")?;
        let add_offset = GeophysicalData::get_attr::<f32>(&sst4, "add_offset")?;

        let data = sst4
            .values_arr::<i16, _>((.., ..))
            .map_err(|r| GeophysicalDataError::Netcdf(r))?;

        let height = data.shape()[0];
        let width = data.shape()[1];

        let mut newdata = vec![0.0f32; width * height];
        let mut valid_pixels = 0;

        for y in 0..height {
            for x in 0..width {
                let scaled_integer_value = data[[y, x]];

                // if scaled_integer_value >= valid_min && scaled_integer_value <= valid_max {
                if scaled_integer_value != fill_value {
                    let geophysical_value =
                        scale_factor * (scaled_integer_value as f32) + add_offset;
                    newdata[y * width + x] = geophysical_value;
                    valid_pixels += 1;
                } else {
                    newdata[y * width + x] = f32::NAN;
                }
            }
        }

        println!("[{}] Valid pixels: {}", name, valid_pixels);

        return Ok(GeophysicalData {
            name: String::from(name),
            data: newdata,
            width,
            height,
        });
    }

    fn apply_bias(&self, bias: &GeophysicalData) -> GeophysicalData {
        let height = self.height;
        let width = self.width;

        let mut newdata = vec![0.0f32; width * height];

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let val = self.data[idx] + bias.data[idx];
                newdata[idx] = val;
            }
        }

        return GeophysicalData {
            name: self.name.clone(),
            data: newdata,
            width,
            height,
        };
    }

    fn compute_minmax(&self) -> (f32, f32) {
        if self.data.is_empty() {
            return (f32::NAN, f32::NAN);
        }

        let mut min_val = f32::MAX;
        let mut max_val = f32::MIN;

        for ele in &self.data {
            min_val = f32::min(min_val, *ele);
            max_val = f32::max(max_val, *ele);
        }

        return (min_val, max_val);
    }

    fn generate_image(&self) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let height = self.height;
        let width = self.width;

        let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

        let (min_val, max_val) = self.compute_minmax();
        for y in 0..height {
            for x in 0..width {
                let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                let val = self.data[y * width + x];
                if val.is_nan() {
                    *pixel = image::Rgba([0, 0, 0, 0]);
                } else {
                    let scale = (val - min_val) / (max_val - min_val);
                    let grayscale = (scale * 255.0) as u8;
                    *pixel = image::Rgba([grayscale, grayscale, grayscale, 255]);
                }
            }
        }

        imgbuf
    }
}

// TODO: allow only nasa subdomains
struct AllowCrossOrigin {}

impl reqwest::redirect::Filter for AllowCrossOrigin {
    fn handle_sensitive_headers(
        &self,
        headers: &mut axum::http::HeaderMap,
        next: &reqwest::Url,
        previous: &[reqwest::Url],
    ) {
        return;
    }
}

#[async_trait]
impl OceanColorService for OceanColorServiceDefault {
    async fn search(
        &self,
        sdate: NaiveDateTime,
        edate: NaiveDateTime,
    ) -> Result<Vec<SearchItem>, Box<dyn std::error::Error>> {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let sdate = sdate.format(fmt).to_string();
        let edate = edate.format(fmt).to_string();

        let mut params = HashMap::new();
        params.insert("results_as_file", "1");
        params.insert("sensor_id", "8");
        params.insert("dtid", "1102");
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

    async fn get(
        &self,
        item: SearchItem,
    ) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
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
        redirect_policy.set_filter(Box::new(AllowCrossOrigin {}));

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
                .ok_or(GeophysicalDataError::FilenameExtractionFromUrlFailed(
                    response.url().clone(),
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