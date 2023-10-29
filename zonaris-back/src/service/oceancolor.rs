use std::{collections::HashMap, sync::Arc};

use axum::async_trait;
use image::ImageBuffer;

#[async_trait]
pub trait OceanColorService {
    async fn get_last_24hours(
        &self,
    ) -> Result<Vec<ImageBuffer<image::Rgba<u8>, Vec<u8>>>, Box<dyn std::error::Error>>;
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
    AttributeNotFound(String),
    Internal,
}

impl GeophysicalData {
    fn get_attr<'a, T>(var: &netcdf::Variable<'a>, name: &str) -> Result<T, GeophysicalDataError>
    where
        T: TryFrom<netcdf::AttrValue>,
    {
        return T::try_from(
            var.attribute(name)
                .ok_or(GeophysicalDataError::AttributeNotFound(String::from(name)))?
                .value()
                .map_err(|r| GeophysicalDataError::Netcdf(r))?,
        )
        .map_err(|r| GeophysicalDataError::Internal); // TODO normal error
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
        let sst4 = geophysical_data.variable(name).unwrap();

        // let valid_min =
        //     i16::try_from(sst4.attribute("valid_min").unwrap().value().unwrap()).unwrap();
        // let valid_max =
        //     i16::try_from(sst4.attribute("valid_max").unwrap().value().unwrap()).unwrap();
        let fill_value = GeophysicalData::get_attr::<i16>(&sst4, "_FillValue")?;

        let scale_factor = GeophysicalData::get_attr::<f32>(&sst4, "scale_factor")?;
        let add_offset = GeophysicalData::get_attr::<f32>(&sst4, "add_offset")?;

        let data = sst4.values_arr::<i16, _>((.., ..)).unwrap();

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
    async fn get_last_24hours(
        &self,
    ) -> Result<Vec<ImageBuffer<image::Rgba<u8>, Vec<u8>>>, Box<dyn std::error::Error>> {
        let tmpdir = tempfile::tempdir()?;

        let mut params = HashMap::new();
        params.insert("results_as_file", "1");
        params.insert("sensor_id", "8");
        params.insert("dtid", "1102");
        // TODO: valid parameters
        params.insert("sdate", "2023-10-28 15:00:00");
        params.insert("edate", "2023-10-28 16:00:00");
        params.insert("subType", "1");
        params.insert("addurl", "1");

        let response = reqwest::Client::new()
            .post("https://oceandata.sci.gsfc.nasa.gov/api/file_search")
            .form(&params)
            .send()
            .await?;

        let urls = response
            .text()
            .await?
            .lines()
            .map(|line| String::from(line))
            .collect::<Vec<_>>();

        let mut result = Vec::new();
        for url in urls {
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

            // CHECK THIS: https://oceancolor.gsfc.nasa.gov/data/download_methods/
            let response = reqwest::ClientBuilder::new()
                .redirect(redirect_policy)
                .default_headers(headers)
                .cookie_provider(cookie_provider.clone())
                .build()?
                .get(url)
                .send()
                .await?;

            let fname = String::from(
                response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("TERRA_MODIS.00000000T000000.L2.SST4.nc"),
            );

            let contents = response.bytes().await?;

            // TODO: it's can be done in RAM but netcdf library doesn't have implementation for reading from memory
            let tmppath = tmpdir.path().join(&fname);
            std::fs::write(&tmppath, contents)?;
            let file = netcdf::open(&tmppath)?;

            let sst4 = GeophysicalData::load_netcdf(&file, "sst4").unwrap(); // TODO: make it without unwrap

            result.push(sst4.generate_image());
        }

        Ok(result)
    }
}
