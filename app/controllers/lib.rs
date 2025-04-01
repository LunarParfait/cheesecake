use axum::Router;
use config::app::app_state::AppState;
use config::create_routes;
use index::IndexController;

pub mod index;

pub fn router() -> Router<AppState> {
    create_routes! {
        IndexController
    }
}
