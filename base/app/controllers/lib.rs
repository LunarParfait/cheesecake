use axum::Router;
use config::app::app_state::AppState;
use config::create_routes;

pub mod index;
use index::IndexController;

pub fn router() -> Router<AppState> {
    create_routes! {
        IndexController
    }
}
