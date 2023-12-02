use std::sync::Arc;

#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use axum::Router;
use tokio_cron_scheduler::JobScheduler;

use crate::model::{satellite::Satellite, satellite_data::SatelliteData};

// repository types
pub type SatelliteRepository =
    Arc<tokio::sync::RwLock<dyn crate::repository::Repository<Satellite> + Send + Sync>>;
pub type SatelliteDataRepository =
    Arc<tokio::sync::RwLock<dyn crate::repository::Repository<SatelliteData> + Send + Sync>>;

// service types
pub type SatelliteService = Arc<dyn crate::service::satellite::SatelliteService + Send + Sync>;
pub type OceanColorService = Arc<dyn crate::service::oceancolor::OceanColorService + Send + Sync>;

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

mod satellite {
    use std::sync::Arc;

    use axum::{extract::State, Json};

    use crate::model::satellite::Satellite;

    use super::AppContext;

    pub async fn get_all(ctx: State<Arc<AppContext>>) -> Json<Vec<Satellite>> {
        Json(ctx.satellite_service.get_all().await)
    }
}

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/satellite/all", axum::routing::get(satellite::get_all))
        .with_state(Arc::new(ctx))
}
