use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::http::header::{self, HeaderMap};
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};
use tokio_util::io::ReaderStream;

use crate::dto::instrument_data::{GetBySatelliteIdRequest, InstrumentDataResponse};
use crate::routes::AppContext;

#[utoipa::path(
    get,
    path = "/data/get",
    params(GetBySatelliteIdRequest),
    responses(
        (status = 200, body=[InstrumentDataResponse])
    )
)]
async fn get_by_satellite_id(
    ctx: State<Arc<AppContext>>,
    request: Query<GetBySatelliteIdRequest>,
) -> Json<Vec<InstrumentDataResponse>> {
    Json(
        ctx.instrument_data_service
            .get_by_satellite_id(request.id)
            .await
            .into_iter()
            .map(|it| InstrumentDataResponse::from(it))
            .collect(),
    )
}

// TODO: https://github.com/OAI/OpenAPI-Specification/issues/2653
// #[utoipa::path(
//     get,
//     path = "/data/assets/{path}",
//     params(
//         ("path" = String, Path, allow_reserved)
//     )
// )]
async fn get_asset(Path(path): Path<String>, _ctx: State<Arc<AppContext>>) -> impl IntoResponse {
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
    let body = axum::body::Body::from_stream(stream);

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

pub fn create_router(ctx: Arc<AppContext>) -> Router {
    return Router::new()
        .route("/data/get", get(get_by_satellite_id))
        .route("/data/assets/*path", get(get_asset))
        .with_state(ctx);
}
