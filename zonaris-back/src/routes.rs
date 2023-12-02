use std::sync::Arc;

#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use axum::Router;
use tokio_cron_scheduler::JobScheduler;

use crate::{
    persistence::{SatelliteDataRepository, SatelliteRepository},
    service::{OceanColorService, SatelliteService},
};

pub struct AppContext {
    #[cfg(feature = "diesel")]
    pub pool: tokio::sync::Mutex<
        deadpool::managed::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    >,

    pub satellite_repository: SatelliteRepository,
    pub satellite_data_repository: SatelliteDataRepository,

    pub satellite_service: SatelliteService,
    pub oceancolor_service: OceanColorService,

    pub job_scheduler: JobScheduler,
}

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    let satellite_router = crate::controller::satellite::create_router(ctx.clone());

    return Router::new().nest(crate::controller::satellite::PATH, satellite_router);
}
