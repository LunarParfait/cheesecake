use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

fn cut_trace(trace: &str) -> &str {
    trace
        .find("axum::handler::Handler")
        .and_then(|i| {
            let temp = trace.get(0..i)?;
            let idx = temp.rfind('\n')?;
            trace.get(0..idx)
        })
        .unwrap_or("[backtrace unavailable]")
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = &self.0;
        let backtrace = source.backtrace().to_string();
        let backtrace = cut_trace(backtrace.as_str());
        write!(f, "INTERNAL SERVER ERROR: {source}. \n{backtrace}")
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;
