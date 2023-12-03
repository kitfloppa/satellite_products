use std::sync::Arc;

#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use axum::Router;
use tokio_cron_scheduler::JobScheduler;

use crate::{
    persistence::{
        model::{
            instrument::Instrument, instrument_data::InstrumentData, oceancolor::OceanColorMapping,
            satellite::Satellite, satellite_instrument::SatelliteInstrument,
        },
        Repository,
    },
    service::{InstrumentDataService, OceanColorService, SatelliteService},
};

pub struct AppContext {
    #[cfg(feature = "diesel")]
    pub pool: tokio::sync::Mutex<
        deadpool::managed::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    >,

    pub satellite_repository: Repository<Satellite>,
    pub instrument_repository: Repository<Instrument>,
    pub satellite_instrument_repository: Repository<SatelliteInstrument>,
    pub instrument_data_repository: Repository<InstrumentData>,

    pub oceancolor_mapping_repository: Repository<OceanColorMapping>,

    pub satellite_service: SatelliteService,
    pub instrument_data_service: InstrumentDataService,
    pub oceancolor_service: OceanColorService,

    pub job_scheduler: JobScheduler,
}

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    let satellite_router = crate::controller::satellite::create_router(ctx.clone());
    let satellite_data_router = crate::controller::instrument_data::create_router(ctx.clone());

    return Router::new()
        .nest(crate::controller::satellite::PATH, satellite_router)
        .nest(
            crate::controller::instrument_data::PATH,
            satellite_data_router,
        );
}
