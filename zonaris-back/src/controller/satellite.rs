use std::sync::Arc;

use axum::Router;
use axum::{extract::State, Json};

use crate::dto::satellite::SatelliteResponse;

use crate::routes::AppContext;

#[utoipa::path(
    get,
    path = "/satellite/all",
    responses(
        (status = 200, body=[SatelliteResponse])
    )
)]
async fn get_all(ctx: State<Arc<AppContext>>) -> Json<Vec<SatelliteResponse>> {
    Json(
        ctx.satellite_service
            .get_all()
            .await
            .into_iter()
            .map(|it| SatelliteResponse::from(it))
            .collect(),
    )
}

pub const PATH: &str = "/satellite";

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    return Router::new()
        .route("/all", axum::routing::get(get_all))
        .with_state(ctx);
}
