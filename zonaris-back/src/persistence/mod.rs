use std::sync::Arc;

use tokio::sync::RwLock;

use self::model::{satellite::Satellite, satellite_data::SatelliteData};

pub mod model;
pub mod repository;

pub type SatelliteRepository =
    Arc<RwLock<dyn self::repository::Repository<Satellite> + Send + Sync>>;

pub type SatelliteDataRepository =
    Arc<RwLock<dyn self::repository::Repository<SatelliteData> + Send + Sync>>;
