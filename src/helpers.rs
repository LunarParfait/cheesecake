use anyhow::{anyhow, bail};
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::process::Command;

/// Looks for Cargo.toml with app = "cheesecake" metadata
pub fn get_app_dir() -> anyhow::Result<PathBuf> {
    let cmd = cargo_metadata::MetadataCommand::new();

    let metadata = match cmd.exec() {
        Ok(m) => m,
        Err(_) => bail!("Not in a cheesecake application"),
    };

    match metadata
        .workspace_metadata
        .get("app")
        .map(|app| app.as_str())
    {
        Some(Some(app)) if app == "cheesecake" => {
            Ok(metadata.workspace_root.into())
        }
        _ => bail!("Not in a cheesecake application"),
    }
}

pub fn normalize_dir(command: &str) -> anyhow::Result<Command> {
    let app_dir = get_app_dir()?;
    let mut command = Command::new(command);
    command.current_dir(app_dir.to_str().unwrap());

    Ok(command)
}

pub fn get_task(name: &str) -> anyhow::Result<String> {
    let _ = get_app_dir()?;

    let cmd = cargo_metadata::MetadataCommand::new();

    let metadata = cmd.exec().unwrap();

    metadata
        .workspace_metadata
        .get("tasks")
        .map(|tasks| tasks.get(name).map(|task| task.as_str()))
        .flatten()
        .flatten()
        .map(|task| task.to_owned())
        .ok_or(anyhow!("Task not found"))
}

pub fn files_recursive(path: PathBuf) -> io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            entries.append(&mut files_recursive(entry.path())?);
        } else {
            entries.push(entry);
        }
    }

    Ok(entries)
}
