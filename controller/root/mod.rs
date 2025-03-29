use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use std::sync::Arc;
use types::app_error::AppResult;
use types::app_state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(index))
}

async fn index() -> AppResult<impl IntoResponse> {
    view::core::render().map_err(|e| e.into())
}
