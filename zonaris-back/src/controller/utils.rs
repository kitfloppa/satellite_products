use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// thx: https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
// https://docs.rs/axum/latest/axum/response/index.html

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
