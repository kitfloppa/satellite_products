use std::sync::Arc;

pub mod satellite;
pub mod celestrak;
pub mod instrument_data;
pub mod oceancolor;

#[cfg(test)]
mod tests;

pub type SatelliteService = Arc<dyn self::satellite::SatelliteService + Send + Sync>;
pub type CelestrakService = Arc<dyn self::celestrak::CelestrakService + Send + Sync>;
pub type InstrumentDataService = Arc<dyn self::instrument_data::InstrumentDataService + Send + Sync>;
pub type OceanColorService = Arc<dyn self::oceancolor::OceanColorService + Send + Sync>;
