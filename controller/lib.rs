use anyhow::anyhow;
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{BoxError, Router};
use types::app_error::AppError;
use types::app_state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower::load_shed;
use tower::load_shed::LoadShedLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub mod root;

pub fn router(max_connections: u32) -> Router<Arc<AppState>> {

    let serve_dir = if cfg!(debug_assertions) {
        "public"
    } else {
        "dist/public"
    };

    Router::new()
        .route("/ping", get(ping))
        .merge(root::router())
        .nest_service("/public", ServeDir::new(serve_dir))
        .layer((HandleErrorLayer::new(handle_error), LoadShedLayer::new()))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(ConcurrencyLimitLayer::new(
            (max_connections as f64 * 1.5) as usize,
        ))
}

async fn ping() -> impl IntoResponse {
    "pong"
}

async fn handle_error(err: BoxError) -> Response<Body> {
    if err.is::<load_shed::error::Overloaded>() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let err: AppError = anyhow!(err).into();
    err.into_response()
}
