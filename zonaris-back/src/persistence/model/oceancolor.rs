use table_macro::Table;

use crate::persistence::repository::{HasId, Id, Reference};

use super::satellite_instrument::SatelliteInstrument;

// NOTE: actually i don't know which type they use on their backend
pub type SensorId = i32;
pub type DataId = i32;

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct OceanColorMapping {
        #[id] id: Option<Id>,
        satellite_instrument_id: Reference<SatelliteInstrument>,
        sensor_id: SensorId,
        data_id: DataId,
    }
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
