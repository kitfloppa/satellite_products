use std::sync::Arc;

use axum::body::StreamBody;
use axum::extract::{Path, Query};
use axum::http::header::{self, HeaderMap};
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};
use tokio_util::io::ReaderStream;

use crate::dto::satellite_data::GetBySatelliteIdRequest;
use crate::persistence::model::satellite_data::SatelliteData;
use crate::routes::AppContext;

async fn get_by_satellite_id(
    ctx: State<Arc<AppContext>>,
    request: Query<GetBySatelliteIdRequest>,
) -> Json<Vec<SatelliteData>> {
    Json(
        ctx.satellite_data_service
            .get_by_satellite_id(request.id)
            .await,
    )
}

async fn get_asset(Path(path): Path<String>, ctx: State<Arc<AppContext>>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => {
            return Err((
                axum::http::StatusCode::NOT_FOUND,
                format!("File not found: {}", err),
            ))
        }
    };

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("image/png").unwrap(),
    ); // TODO: unwrap
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str("inline").unwrap(),
    ); // TODO: unwrap

    Ok((headers, body))
}

pub const PATH: &str = "/data";

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    return Router::new()
        .route("/get", get(get_by_satellite_id))
        .route("/assets/*path", get(get_asset))
        .with_state(ctx);
}
