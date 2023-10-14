use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Satellite {
    name: String,
    tle1: String,
    tle2: String,
}

impl Satellite {
    pub fn new(name: &str, tle1: &str, tle2: &str) -> Satellite {
        Satellite {
            name: String::from(name),
            tle1: String::from(tle1),
            tle2: String::from(tle2),
        }
    }
}
