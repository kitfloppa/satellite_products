use table_macro::Table;

use crate::persistence::repository::{HasId, Id, Reference};

use super::satellite_instrument::SatelliteInstrument;

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct InstrumentData {
        #[id] id: Option<Id>,
        satellite_instrument_id: Reference<SatelliteInstrument>,
        path: String,
    }
}

impl InstrumentData {
    pub fn new(satellite_instrument_id: i32, path: String) -> Self {
        return Self {
            id: None,
            satellite_instrument_id: Reference::new(satellite_instrument_id),
            path,
        };
    }
}
