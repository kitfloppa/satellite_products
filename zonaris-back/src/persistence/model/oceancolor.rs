use table_macro::Table;

use crate::persistence::repository::{HasId, Id};

// NOTE: actually i don't know which type they use on their backend
pub type SensorId = i32;
pub type DataId = i32;

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct OceanColorMapping {
        id: Option<Id>,
        satellite_instrument_id: Id, // reference SatelliteInstrument.id
        sensor_id: SensorId,
        data_id: DataId,
    }
}

impl OceanColorMapping {
    pub fn new(satellite_instrument_id: Id, sensor_id: SensorId, data_id: DataId) -> Self {
        return Self {
            id: None,
            satellite_instrument_id,
            sensor_id,
            data_id,
        };
    }
}

impl HasId for OceanColorMapping {
    fn get_id(&self) -> Option<Id> {
        return self.id;
    }

    fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}
