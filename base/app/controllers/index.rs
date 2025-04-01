use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use config::app::app_state::AppState;
use config::cheesecake::app_error::AppResult;
use config::cheesecake::controller::Controller;
use entities::prelude::*;
use entities::user;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{EntityTrait, PaginatorTrait};

pub struct IndexController;

impl Controller for IndexController {
    fn router() -> (&'static str, Router<AppState>) {
        let router = Router::new().route("/", get(Self::show));

        ("/", router)
    }
}

impl IndexController {
    async fn show(
        State(state): State<AppState>,
    ) -> AppResult<impl IntoResponse> {
        let cnt = User::find().count(&state.pool).await?;
        let new_user = user::ActiveModel {
            id: NotSet,
            name: Set(format!("user {cnt}")),
        };
        User::insert(new_user).exec(&state.pool).await?;

        let users = User::find().all(&state.pool).await?;
        Ok(views::index::render(users)?)
    }
}
