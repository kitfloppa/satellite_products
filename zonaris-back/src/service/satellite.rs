use axum::async_trait;

use crate::model::satellite::Satellite;

#[async_trait]
pub trait SatelliteService {
    async fn get_all(&self) -> Vec<Satellite>;
}

pub struct SatelliteServiceMock {
    satellites: Vec<Satellite>,
}

impl SatelliteServiceMock {
    pub fn new(satellites: Vec<Satellite>) -> SatelliteServiceMock {
        SatelliteServiceMock { satellites }
    }
}

#[async_trait]
impl SatelliteService for SatelliteServiceMock {
    async fn get_all(&self) -> Vec<Satellite> {
        return self.satellites.to_vec();
    }
}
