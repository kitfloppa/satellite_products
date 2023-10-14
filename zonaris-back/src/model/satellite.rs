use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Satellite {
    name: String,
    lat: f32,
    lng: f32,
}

impl Satellite {
    pub fn new(name: &str, lat: f32, lng: f32) -> Satellite {
        Satellite {
            name: String::from(name),
            lat,
            lng,
        }
    }
}
