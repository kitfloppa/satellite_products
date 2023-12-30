use std::sync::Arc;

use axum::Router;
use axum::{extract::State, Json};

use crate::dto::satellite::SatelliteResponse;

use crate::routes::AppContext;

use super::utils::AppError;

const PATH_ALL: &str = "/satellite/all";

#[utoipa::path(
    get,
    path = PATH_ALL,
    responses(
        (status = 200, body=[SatelliteResponse])
    )
)]
async fn get_all(ctx: State<Arc<AppContext>>) -> Result<Json<Vec<SatelliteResponse>>, AppError> {
    return Ok(Json(
        ctx.satellite_service
            .get_all()
            .await?
            .into_iter()
            .map(|it| SatelliteResponse::from(it))
            .collect(),
    ));
}

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    return Router::new()
        .route(PATH_ALL, axum::routing::get(get_all))
        .with_state(ctx);
}
