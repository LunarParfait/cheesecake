use std::sync::Arc;
use axum::Router;
use model::app_state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
}
