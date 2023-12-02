use crate::repository::{HasId, Id};

#[derive(Clone)]
pub struct SatelliteData {
    id: Option<Id>,
    satellite_id: i32,
    path: String,
}

impl HasId for SatelliteData {
    fn get_id(&self) -> Option<Id> {
        return self.id;
    }

    fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}

impl SatelliteData {
    pub fn new(satellite_id: i32, path: String) -> SatelliteData {
        return SatelliteData {
            id: None,
            satellite_id,
            path,
        };
    }
}
