use anyhow::anyhow;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize)]
pub struct Config {
    name: Option<String>,
    version: Option<String>,
    cheesecake_version: String,
    tasks: HashMap<String, String>,
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    find_ancestor(std::env::current_dir()?, "cheesecake.toml")?
        .ok_or(anyhow!("Not in a cheesecake application"))
        .map(|entry| entry.path())
}

/// Looks for cheesecake.toml
pub fn get_app_dir() -> anyhow::Result<PathBuf> {
    get_config_path().map(|mut path| {
        path.pop();
        path
    })
}

pub fn normalize_dir(command: &str) -> anyhow::Result<Command> {
    let app_dir = get_app_dir()?;
    let mut command = Command::new(command);
    command.current_dir(app_dir.to_str().unwrap());

    Ok(command)
}

pub fn get_config() -> anyhow::Result<Config> {
    let config_path = get_config_path()?;
    let mut contents = String::new();
    File::open(config_path)?.read_to_string(&mut contents)?;

    Ok(toml::from_str(&contents)?)
}

pub fn get_task(name: &str) -> anyhow::Result<String> {
    get_config()?
        .tasks
        .get(name)
        .map(|s| s.to_owned())
        .ok_or(anyhow!("Task not found"))
}

pub fn find_ancestor(
    mut path: PathBuf,
    filename: &str,
) -> io::Result<Option<DirEntry>> {
    while let Some(parent) = path.parent() {
        for entry in fs::read_dir(path.clone())? {
            let entry = entry?;

            if entry.file_name().to_str().unwrap() == filename {
                return Ok(Some(entry));
            }
        }

        path = parent.into();
    }

    return Ok(None);
}

pub fn files_in_dir_recursive(path: PathBuf) -> io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            entries.append(&mut files_in_dir_recursive(entry.path())?);
        } else {
            entries.push(entry);
        }
    }

    Ok(entries)
}
