use axum::extract::State;
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use std::sync::Arc;
use types::app_error::AppResult;
use types::app_state::AppState;
use sea_orm::*;
use database::entities::{prelude::*, user};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(index))
}

async fn index(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let cnt = User::find().count(&state.pool).await?;
    let new_user = user::ActiveModel {
        id: NotSet,
        name: Set(format!("user {cnt}")),
    };
    User::insert(new_user).exec(&state.pool).await?;

    let users = User::find().all(&state.pool).await?;
    Ok(view::root::render(users)?)
}
