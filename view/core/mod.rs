use super::AppTemplate;
use crate::RenderResult;
use serde::Serialize;

#[derive(Serialize, Default)]
struct Template {}

pub fn render() -> RenderResult {
    Template::default().render("index.html")
}
