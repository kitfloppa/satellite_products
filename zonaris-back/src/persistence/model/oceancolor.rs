use table_macro::{Property, Table};

use crate::persistence::repository::{Id, Reference};

use super::satellite_instrument::SatelliteInstrument;

// NOTE: actually i don't know which type they use on their backend
pub type SensorId = i32;
pub type DataId = i32;

#[derive(Clone, Table, Property)]
pub struct OceanColorMapping {
    #[id]
    #[none]
    id: Option<Id>,
    satellite_instrument_id: Reference<SatelliteInstrument>,
    sensor_id: SensorId,
    data_id: DataId,
}

impl OceanColorMapping {
    pub fn new(satellite_instrument_id: Id, sensor_id: SensorId, data_id: DataId) -> Self {
        return Self {
            id: None,
            satellite_instrument_id: Reference::new(satellite_instrument_id),
            sensor_id,
            data_id,
        };
    }
}
