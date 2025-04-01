use std::path::PathBuf;

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
