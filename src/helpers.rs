use std::path::PathBuf;
use std::process::{Child, Command};
use anyhow::anyhow;

/// Looks for Cargo.toml with app = "cheesecake" metadata
pub fn get_app_dir() -> Option<PathBuf> {
    let cmd = cargo_metadata::MetadataCommand::new();

    let metadata = match cmd.exec() {
        Ok(m) => m,
        Err(_) => return None,
    };

    match metadata
        .workspace_metadata
        .get("app")
        .map(|app| app.as_str())
    {
        Some(Some(app)) if app == "cheesecake" => {
            Some(metadata.workspace_root.into())
        }
        _ => None,
    }
}

pub fn normalize_dir(command: &str) -> anyhow::Result<Command> {
    let app_dir = get_app_dir().ok_or(anyhow!("Not in a cheesecake app"))?;
    let mut command = Command::new(command);
    command
        .current_dir(app_dir.to_str().unwrap());

    Ok(command)
}
