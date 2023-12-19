use table_macro::Table;

use crate::persistence::repository::{HasId, Id};

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct InstrumentData {
        #[id] id: Option<Id>,
        satellite_instrument_id: i32, // reference SatelliteInstrument.id
        path: String,
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
