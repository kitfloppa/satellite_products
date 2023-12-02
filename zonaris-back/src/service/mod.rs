use std::sync::Arc;

pub mod oceancolor;
pub mod satellite;

pub type SatelliteService = Arc<dyn self::satellite::SatelliteService + Send + Sync>;
pub type OceanColorService = Arc<dyn self::oceancolor::OceanColorService + Send + Sync>;
