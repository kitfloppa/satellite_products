use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::http::header::{self, HeaderMap};
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};
use tokio_util::io::ReaderStream;

use crate::dto::instrument_data::{
    GetAssetRequest, GetBySatelliteIdRequest, InstrumentDataResponse,
};
use crate::routes::AppContext;

use super::utils::to_internal;

const PATH_GET: &str = "/data/get";
const PATH_GET_ASSET: &str = "/data/get_asset";

#[utoipa::path(
    get,
    path = PATH_GET,
    params(GetBySatelliteIdRequest),
    responses(
        (status = 200, body=[InstrumentDataResponse])
    )
)]
async fn get_by_satellite_id(
    ctx: State<Arc<AppContext>>,
    request: Query<GetBySatelliteIdRequest>,
) -> Result<Json<Vec<InstrumentDataResponse>>, StatusCode> {
    return Ok(Json(
        ctx.instrument_data_service
            .get_by_satellite_id(request.id)
            .await
            .map_err(to_internal)?
            .into_iter()
            .map(|it| InstrumentDataResponse::from(it))
            .collect(),
    ));
}

#[utoipa::path(
    get,
    path = PATH_GET_ASSET,
    params(GetAssetRequest),
    responses(
        (status = 200),
        (status = 404)
    )
)]
async fn get_asset(
    _ctx: State<Arc<AppContext>>,
    request: Query<GetAssetRequest>,
) -> impl IntoResponse {
    let file = match tokio::fs::File::open(&request.path).await {
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
        .route(PATH_GET, get(get_by_satellite_id))
        .route(PATH_GET_ASSET, get(get_asset))
        .with_state(ctx);
}
