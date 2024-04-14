use table_macro::{Property, Table};

use crate::persistence::repository::{Id, Reference};

use super::satellite_instrument::SatelliteInstrument;

#[derive(Clone, Table, Property)]
pub struct InstrumentData {
    #[id]
    #[none]
    id: Option<Id>,
    satellite_instrument_id: Reference<SatelliteInstrument>,
    path: String,
}

impl InstrumentData {
    pub fn new(satellite_instrument_id: Id, path: String) -> Self {
        return Self {
            id: None,
            satellite_instrument_id: Reference::new(satellite_instrument_id),
            path,
        };
    }
}
