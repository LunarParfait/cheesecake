use super::app_state::AppState;
use crate::velvet::app_error::AppError;
use anyhow::anyhow;
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::{BoxError, Router};
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower::load_shed::{self, LoadShedLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub fn router(app_state: AppState) -> Router<AppState> {
    let max_connections = app_state
        .pool
        .get_sqlite_connection_pool()
        .options()
        .get_max_connections();

    Router::new()
        .layer((HandleErrorLayer::new(handle_error), LoadShedLayer::new()))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(ConcurrencyLimitLayer::new(
            (max_connections as f64 * 1.5) as usize,
        ))
}

async fn handle_error(err: BoxError) -> Response<Body> {
    if err.is::<load_shed::error::Overloaded>() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let err: AppError = anyhow!(err).into();
    err.into_response()
}
