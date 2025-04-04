use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use config::app::app_state::AppState;
use config::cheesecake::app_error::AppResult;
use config::cheesecake::controller::Controller;

pub struct IndexController;

impl Controller for IndexController {
    fn router() -> (&'static str, Router<AppState>) {
        let router = Router::new().route("/", get(Self::show));

        ("/", router)
    }
}

impl IndexController {
    async fn show() -> AppResult<impl IntoResponse> {
        Ok(views::index::render()?)
    }
}
