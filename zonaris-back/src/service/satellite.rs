use anyhow::Result;
use axum::async_trait;

use crate::persistence::{model::satellite::Satellite, Repository};

#[async_trait]
pub trait SatelliteService {
    async fn get_all(&self) -> Result<Vec<Satellite>>;
}

pub struct SatelliteServiceDefault {
    satellite_repository: Repository<Satellite>,
}

impl SatelliteServiceDefault {
    pub fn new(satellite_repository: Repository<Satellite>) -> SatelliteServiceDefault {
        SatelliteServiceDefault {
            satellite_repository,
        }
    }
}

#[async_trait]
impl SatelliteService for SatelliteServiceDefault {
    async fn get_all(&self) -> Result<Vec<Satellite>> {
        let satellite_repository = self.satellite_repository.read().await;
        return satellite_repository.get_all().await;
    }
}
