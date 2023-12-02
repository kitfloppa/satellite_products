use std::sync::Arc;

pub mod oceancolor;
pub mod satellite;
pub mod satellite_data;

// TODO: i dont sure but maybe all services should be under RwLock

pub type SatelliteService = Arc<dyn self::satellite::SatelliteService + Send + Sync>;
pub type SatelliteDataService = Arc<dyn self::satellite_data::SatelliteDataService + Send + Sync>;
pub type OceanColorService = Arc<dyn self::oceancolor::OceanColorService + Send + Sync>;
