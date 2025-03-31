use anyhow::anyhow;
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
#[cfg(debug_assertions)]
use axum::extract::ws::WebSocket;
#[cfg(debug_assertions)]
use axum::extract::ws::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
#[cfg(debug_assertions)]
use axum::routing::any;
use axum::routing::get;
use axum::{BoxError, Router};
use std::sync::Arc;
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower::load_shed;
use tower::load_shed::LoadShedLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use types::app_error::AppError;
use types::app_state::AppState;

pub mod root;

pub fn router(max_connections: u32) -> Router<Arc<AppState>> {
    let serve_dir = if cfg!(debug_assertions) {
        "public"
    } else {
        "dist/public"
    };

    #[cfg(debug_assertions)]
    let router = Router::new().route("/dev-server", any(dev_server));

    #[cfg(not(debug_assertions))]
    let router = Router::new();

    router
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

#[cfg(debug_assertions)]
async fn dev_server(ws: WebSocketUpgrade) -> impl IntoResponse {

    ws.on_upgrade(dev_socket)
}

#[cfg(debug_assertions)]
async fn dev_socket(mut socket: WebSocket) {
    use axum::extract::ws::Message;

    let mut receiver = view::HOTWATCH_CHANNEL.0.subscribe();

    while receiver.changed().await.is_ok() {
        if socket.send(Message::binary("")).await.is_err() {
            return;
        }
    }
}

async fn handle_error(err: BoxError) -> Response<Body> {
    if err.is::<load_shed::error::Overloaded>() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let err: AppError = anyhow!(err).into();
    err.into_response()
}
