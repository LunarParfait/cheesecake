use config::cheesecake::view::{AppTemplate, RenderResult};
use serde::Serialize;

#[derive(Serialize, Default)]
struct Template {}

pub fn render() -> RenderResult {
    Template {}.render("index.html")
}
