use table_macro::Table;

use crate::persistence::repository::{HasId, Id};

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct SatelliteInstrument {
        #[id] id: Option<Id>,
        satellite_id: Id,  // reference Satellite.id
        instrument_id: Id, // reference Instrument.id
    }
}

impl SatelliteInstrument {
    pub fn new(satellite_id: Id, instrument_id: Id) -> Self {
        return Self {
            id: None,
            satellite_id,
            instrument_id,
        };
    }
}
