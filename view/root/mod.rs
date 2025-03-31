use crate::RenderResult;
use anyhow::bail;
use database::entities::user;
use serde::Serialize;

#[derive(Serialize, Default)]
struct Template {
    users: Vec<user::Model>,
}

pub fn render(users: Vec<user::Model>) -> RenderResult {
    bail!("meow")
}
