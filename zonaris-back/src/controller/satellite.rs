use std::sync::Arc;

use axum::Router;
use axum::{extract::State, Json};

use crate::persistence::model::satellite::Satellite;

use crate::routes::AppContext;

async fn get_all(ctx: State<Arc<AppContext>>) -> Json<Vec<Satellite>> {
    Json(ctx.satellite_service.get_all().await)
}

pub const PATH: &str = "/controller";

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    return Router::new()
        .route(
            "/all",
            axum::routing::get(crate::controller::satellite::get_all),
        )
        .with_state(ctx);
}
