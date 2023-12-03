use axum::async_trait;

use crate::persistence::{model::satellite::Satellite, Repository};

#[async_trait]
pub trait SatelliteService {
    async fn get_all(&self) -> Vec<Satellite>;
}

pub struct SatelliteServiceMock {
    satellite_repository: Repository<Satellite>,
}

impl SatelliteServiceMock {
    pub fn new(satellite_repository: Repository<Satellite>) -> SatelliteServiceMock {
        SatelliteServiceMock {
            satellite_repository,
        }
    }
}

#[async_trait]
impl SatelliteService for SatelliteServiceMock {
    async fn get_all(&self) -> Vec<Satellite> {
        let satellite_repository = self.satellite_repository.read().await;
        return satellite_repository.get_all().await;
    }
}
