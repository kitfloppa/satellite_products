use table_macro::{Property, Table};

use crate::persistence::repository::{Id, Reference};

use super::{instrument::Instrument, satellite::Satellite};

#[derive(Clone, Table, Property)]
pub struct SatelliteInstrument {
    #[id]
    #[none]
    id: Option<Id>,
    satellite_id: Reference<Satellite>,
    instrument_id: Reference<Instrument>,
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
