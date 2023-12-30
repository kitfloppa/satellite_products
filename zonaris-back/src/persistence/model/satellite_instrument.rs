use table_macro::Table;

use crate::persistence::repository::{HasId, Id, Reference};

use super::{instrument::Instrument, satellite::Satellite};

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct SatelliteInstrument {
        #[id] id: Option<Id>,
        satellite_id: Reference<Satellite>,
        instrument_id: Reference<Instrument>,
    }
}

impl SatelliteInstrument {
    pub fn new(satellite_id: Id, instrument_id: Id) -> Self {
        return Self {
            id: None,
            satellite_id: Reference::new(satellite_id),
            instrument_id: Reference::new(instrument_id),
        };
    }
}
