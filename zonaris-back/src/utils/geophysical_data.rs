use anyhow::{anyhow, Result};
use image::ImageBuffer;
use log::trace;

pub struct GeophysicalData {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl GeophysicalData {
    fn get_attr<'a, T>(var: &netcdf::Variable<'a>, name: &str) -> Result<T>
    where
        T: TryFrom<netcdf::AttributeValue, Error = netcdf::Error>,
    {
        return Ok(T::try_from(
            var.attribute(name)
                .ok_or(anyhow!("following attribute not found: {}", name))?
                .value()?,
        )?);
    }

    pub fn load_netcdf(file: &netcdf::File, name: &str) -> Result<GeophysicalData> {
        let geophysical_data = file
            .group("geophysical_data")?
            .ok_or(anyhow!("failed to fetch 'geophysical_data' group"))?;

        let sst4 = geophysical_data
            .variable(name)
            .ok_or(anyhow!("variable not found {}", name))?;

        let fill_value = GeophysicalData::get_attr::<i16>(&sst4, "_FillValue")?;

        let scale_factor = GeophysicalData::get_attr::<f32>(&sst4, "scale_factor")?;
        let add_offset = GeophysicalData::get_attr::<f32>(&sst4, "add_offset")?;

        let data = sst4.values_arr::<i16, _>((.., ..))?;

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

        trace!("[{}] Valid pixels: {}", name, valid_pixels);

        return Ok(GeophysicalData {
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

    pub fn generate_image(&self) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
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
