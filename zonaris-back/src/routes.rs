use std::sync::Arc;

use axum::Router;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

pub type SatelliteService = Arc<dyn crate::service::satellite::SatelliteService + Send + Sync>;

pub struct AppContext {
    pub pool:
        deadpool::managed::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    pub satellite_service: SatelliteService,
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