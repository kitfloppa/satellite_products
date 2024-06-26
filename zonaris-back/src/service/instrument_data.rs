use std::collections::HashSet;

use anyhow::Result;
use async_trait::async_trait;

use crate::persistence::{
    model::{instrument_data::InstrumentData, satellite_instrument::SatelliteInstrument},
    repository::{HasId, Id},
    Repository,
};

#[async_trait]
pub trait InstrumentDataService {
    async fn add_data(&self, data: InstrumentData) -> Result<bool>;
    async fn get_by_id(&self, id: Id) -> Result<Option<InstrumentData>>;
    async fn get_by_satellite_id(&self, id: Id) -> Result<Vec<InstrumentData>>;
}

pub struct InstrumentDataServiceDefault {
    satellite_instrument_repository: Repository<SatelliteInstrument>,
    instrument_data_repository: Repository<InstrumentData>,
}

impl InstrumentDataServiceDefault {
    pub fn new(
        satellite_instrument_repository: Repository<SatelliteInstrument>,
        instrument_data_repository: Repository<InstrumentData>,
    ) -> Self {
        Self {
            satellite_instrument_repository,
            instrument_data_repository,
        }
    }
}

#[async_trait]
impl InstrumentDataService for InstrumentDataServiceDefault {
    async fn add_data(&self, data: InstrumentData) -> Result<bool> {
        let mut satellite_data_repository = self.instrument_data_repository.write().await;
        satellite_data_repository.add(data).await?;
        return Ok(true);
    }

    async fn get_by_id(&self, id: Id) -> Result<Option<InstrumentData>> {
        return Ok(self.instrument_data_repository.read().await.get(id).await?);
    }

    // TODO: it's should be done on repository level and repository should give public api for this
    async fn get_by_satellite_id(&self, satellite_id: Id) -> Result<Vec<InstrumentData>> {
        let satellite_instrument_ids = {
            let lock = self.satellite_instrument_repository.read().await;
            lock.get_all()
                .await?
                .into_iter()
                .filter(|it| *it.get_satellite_id() == satellite_id)
                .filter_map(|it| it.get_id())
                .collect::<HashSet<_>>()
        };

        let lock = self.instrument_data_repository.read().await;
        return Ok(lock
            .get_all()
            .await?
            .into_iter()
            .filter(|it| satellite_instrument_ids.contains(&it.get_satellite_instrument_id()))
            .collect());
    }
}
