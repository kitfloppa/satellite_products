use async_trait::async_trait;

use crate::persistence::{
    model::satellite_data::SatelliteData, repository::Id, SatelliteDataRepository,
};

#[async_trait]
pub trait SatelliteDataService {
    async fn add_data(&self, data: SatelliteData) -> bool;
    async fn get_by_satellite_id(&self, id: Id) -> Vec<SatelliteData>;
}

pub struct SatelliteDataServiceDefault {
    satellite_data_repository: SatelliteDataRepository,
}

impl SatelliteDataServiceDefault {
    pub fn new(satellite_data_repository: SatelliteDataRepository) -> Self {
        Self {
            satellite_data_repository,
        }
    }
}

#[async_trait]
impl SatelliteDataService for SatelliteDataServiceDefault {
    async fn add_data(&self, data: SatelliteData) -> bool {
        let mut satellite_data_repository = self.satellite_data_repository.write().await;
        return satellite_data_repository.add(data).await;
    }

    async fn get_by_satellite_id(&self, satellite_id: Id) -> Vec<SatelliteData> {
        let satellite_data_repository = self.satellite_data_repository.read().await;
        // TODO: it's should be done on repository level and repository should give public api for this
        return satellite_data_repository
            .get_all()
            .await
            .into_iter()
            .filter(|it| it.satellite_id == satellite_id)
            .collect();
    }
}
