use std::sync::Arc;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use model::app_error::AppResult;
use model::app_state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index))
}

async fn index() -> AppResult<impl IntoResponse> {
    view::core::render().map_err(|e| e.into())
}
