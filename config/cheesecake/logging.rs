use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

use super::env::BASE_ENV;

pub async fn init_logging() {
    if cfg!(debug_assertions) {
        println!("\nInitializing logging...\n");
    }

    let log_dir = log_directory().await;

    // TODO: implement this thing
    // Filtering crates
    let filtered = vec![]; // You can had here any crates that are too verbose
    let env_filter = filter(&filtered);

    let now = chrono::Local::now();
    let filename = format!("{}.log", now.format("%Y-%m-%dZ%H:%M:%S"));
    let file = File::create(log_dir.join(filename)).unwrap();
    let latest = File::create(log_dir.join("latest.log")).unwrap();

    let file1_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_thread_ids(true)
        .with_writer(Arc::new(file));

    let file2_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_thread_ids(true)
        .with_writer(Arc::new(latest));

    let stdout_log = tracing_subscriber::fmt()
        .pretty()
        .with_thread_ids(true)
        .with_env_filter(env_filter)
        .finish();

    let subscriber = stdout_log.with(file1_log).with(file2_log);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    if cfg!(debug_assertions) {
        println!("Success!\n");
        println!("-------------------------------------------------------\n");
    }
}

#[must_use]
pub fn canonicalize_unexistent(s: &Path) -> Option<PathBuf> {
    for p in s.ancestors() {
        if let Ok(path) = (|| {
            let canonical = p.canonicalize()?;
            let stripped = s.strip_prefix(p)?;
            Ok::<PathBuf, anyhow::Error>(canonical.join(stripped))
        })() {
            return Some(path);
        };
    }
    None
}

// This function creates the log directory and returns its path.
async fn log_directory() -> PathBuf {
    let canonical = canonicalize_unexistent(&BASE_ENV.log_directory)
        .unwrap_or_else(|| panic!("Failed to canonicalize path!"));

    tokio::fs::create_dir_all(&canonical)
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Failed to create canonical directory: {e}. Path: {canonical:?}"
            )
        });

    canonical
}

// This function creates the filter for the logging system.
fn filter(filter_entries: &[&str]) -> EnvFilter {
    if cfg!(debug_assertions) {
        println!("Defining EnvFilter...\n");
    }

    let filter = EnvFilter::builder()
        .with_default_directive(BASE_ENV.log_level.into())
        .from_env()
        .unwrap_or_else(|e| {
            panic!("Invalid directives for tracing subscriber: {e}.")
        });

    filter_entries.iter().fold(filter, |acc, s| {
        acc.add_directive(format!("{s}=warn").parse().unwrap())
    })
}
