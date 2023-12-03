use serde::Serialize;

use crate::persistence::repository::{HasId, Id};

crate::pub_fields! {
    #[derive(Clone)]
    #[derive(Serialize)]
    struct InstrumentData {
        id: Option<Id>,
        satellite_instrument_id: i32, // reference SatelliteInstrument.id
        path: String,
    }
}

impl HasId for InstrumentData {
    fn get_id(&self) -> Option<Id> {
        return self.id;
    }

    fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}

impl InstrumentData {
    pub fn new(satellite_instrument_id: i32, path: String) -> Self {
        return Self {
            id: None,
            satellite_instrument_id,
            path,
        };
    }
}
