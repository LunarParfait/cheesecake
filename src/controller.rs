use clap::Subcommand;
use std::fs::File;
use std::io::{Read, Write};

use crate::helpers::get_app_dir;

#[derive(Debug, Subcommand)]
pub enum ControllerCommand {
    /// Creates new controller
    Create { name: String, handlers: Vec<String> },
    /// Deletes controller
    Delete { name: String },
}

pub fn handle_command(command: ControllerCommand) -> anyhow::Result<()> {
    match command {
        ControllerCommand::Create { name, handlers } => {
            create_controller(name, handlers)
        }
        ControllerCommand::Delete { name } => delete_controller(name),
    }
}

const CONTROLLER_PATH: &'static str = "app/controllers";

const CONTROLLER_IMPORTS: &'static str = r#"use axum::response::IntoResponse;
use config::cheesecake::app_error::AppResult;

"#;

const CONTROLLER_HANDLER: &'static str = r#"pub async fn {}() -> AppResult<impl IntoResponse> {
    Ok("Hello from {}")
}

"#;

const CONTROLLER_MOD: &'static str = "pub mod {};\n";

fn create_controller(
    name: String,
    handlers: Vec<String>,
) -> anyhow::Result<()> {
    let app_dir = get_app_dir()?;
    let controller_dir = app_dir.join(CONTROLLER_PATH);
    let librs = controller_dir.clone().join("lib.rs");
    let controller_file = controller_dir.join(format!("{}.rs", name));

    let mut file = File::create_new(controller_file.clone())?;
    file.write_all(CONTROLLER_IMPORTS.as_bytes())?;

    for handler in handlers.into_iter() {
        file.write_all(CONTROLLER_HANDLER.replace("{}", &handler).as_bytes())?;
    }

    let mut librs_file = File::open(librs.clone())?;
    let mut contents = Vec::new();
    librs_file.read_to_end(&mut contents)?;
    let mut contents = String::from_utf8(contents)?;

    let modstr = CONTROLLER_MOD.replace("{}", &name);

    if contents.find(&modstr).is_some() {
        return Ok(());
    }

    match contents.find("\n\n") {
        Some(pos) => {
            contents.insert_str(pos+2, &modstr);
        }
        None => {
            contents.push_str(&modstr);
        }
    }

    File::create(librs.clone())?.write_all(contents.as_bytes())?;

    Ok(())
}

fn delete_controller(name: String) -> anyhow::Result<()> {
    let app_dir = get_app_dir()?;
    let controller_dir = app_dir.join(CONTROLLER_PATH);
    let librs = controller_dir.clone().join("lib.rs");
    let controller_file = controller_dir.join(format!("{}.rs", name));

    std::fs::remove_file(controller_file.clone())?;

    let mut librs_file = File::open(librs.clone())?;
    let mut contents = Vec::new();
    librs_file.read_to_end(&mut contents)?;
    let contents = String::from_utf8(contents)?;

    let modstr = CONTROLLER_MOD.replace("{}", &name);
    let contents = contents.split(&modstr).collect::<Vec<_>>().join("");
    File::create(librs.clone()).unwrap().write_all(contents.as_bytes())?;

    Ok(())
}
