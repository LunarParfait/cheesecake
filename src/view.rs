use std::fs::File;
use std::io::{Read, Write};

use clap::Subcommand;

use crate::helpers::get_app_dir;

#[derive(Debug, Subcommand)]
pub enum ViewCommand {
    /// Creates new view
    Create { name: String },
    /// Deletes view
    Delete { name: String },
}

pub fn handle_command(command: ViewCommand) -> anyhow::Result<()> {
    match command {
        ViewCommand::Create { name } => create_view(name),
        ViewCommand::Delete { name } => delete_view(name),
    }
}

const VIEW_PATH: &'static str = "app/views";

const VIEW_RS: &'static str = r#"use config::cheesecake::view::{AppTemplate, RenderResult};
use serde::Serialize;

#[derive(Serialize, Default)]
struct Template {}

pub fn render() -> RenderResult {
    Template {}.render("VIEW_NAME.html")
}
"#;

const VIEW_MOD: &'static str = "pub mod {};\n";

const VIEW_TEMPLATE_PATH: &'static str = "resources/templates";

const VIEW_HTML: &'static str = r#"{% extends "base.html" %}
{% block head %}
<title>VIEW_NAME</title>
{% endblock head %}

{% block body %}
<h1>Hello from VIEW_NAME!</h1>
{% endblock body %}
"#;

pub fn create_view(name: String) -> anyhow::Result<()> {
    let app_dir = get_app_dir()?;
    let view_dir = app_dir.join(VIEW_PATH);
    let template_dir = app_dir.join(VIEW_TEMPLATE_PATH);
    let viewrs_file = view_dir.join(format!("{}.rs", name));
    let viewtemplate_file = template_dir.join(format!("{}.html", name));
    let librs = view_dir.join("lib.rs");

    File::create_new(viewrs_file)?
        .write_all(VIEW_RS.replace("VIEW_NAME", &name).as_bytes())?;

    File::create_new(viewtemplate_file)?
        .write_all(VIEW_HTML.replace("VIEW_NAME", &name).as_bytes())?;

    let mut librs_file = File::open(librs.clone())?;
    let mut contents = Vec::new();
    librs_file.read_to_end(&mut contents)?;
    let mut contents = String::from_utf8(contents)?;

    let modstr = VIEW_MOD.replace("{}", &name);

    if contents.find(&modstr).is_some() {
        return Ok(());
    }

    contents.push_str(&modstr);
    File::create(librs.clone())?.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn delete_view(name: String) -> anyhow::Result<()> {
    let app_dir = get_app_dir()?;
    let view_dir = app_dir.join(VIEW_PATH);
    let template_dir = app_dir.join(VIEW_TEMPLATE_PATH);
    let viewrs_file = view_dir.join(format!("{}.rs", name));
    let viewtemplate_file = template_dir.join(format!("{}.html", name));
    let librs = view_dir.join("lib.rs");

    std::fs::remove_file(viewrs_file.clone())?;
    std::fs::remove_file(viewtemplate_file.clone())?;

    let mut librs_file = File::open(librs.clone())?;
    let mut contents = Vec::new();
    librs_file.read_to_end(&mut contents)?;
    let contents = String::from_utf8(contents)?;

    let modstr = VIEW_MOD.replace("{}", &name);
    let contents = contents.split(&modstr).collect::<Vec<_>>().join("");
    File::create(librs.clone()).unwrap().write_all(contents.as_bytes())?;

    Ok(())
}
