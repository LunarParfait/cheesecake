use super::AppTemplate;
use crate::RenderResult;
use database::entities::user;
use serde::Serialize;

#[derive(Serialize, Default)]
struct Template {
    users: Vec<user::Model>,
}

pub fn render(users: Vec<user::Model>) -> RenderResult {
    Template { users }.render("index.html")
}
