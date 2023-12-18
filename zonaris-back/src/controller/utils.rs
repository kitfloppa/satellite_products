use axum::http::StatusCode;
use log::error;

pub fn to_internal(err: anyhow::Error) -> StatusCode {
    error!("{}\n{}", err, err.backtrace());
    return StatusCode::INTERNAL_SERVER_ERROR;
}
