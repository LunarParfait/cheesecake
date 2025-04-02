use crate::helpers::{get_app_dir, get_task, normalize_dir};
use anyhow::{anyhow, bail};
use git2::{Repository, RepositoryInitOptions};
use indicatif::ProgressBar;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{env, fs, thread};

pub const REPO_URL: &'static str =
    "https://github.com/LunarParfait/cheesecake-base.git";

pub fn new_app(name: String) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Downloading scaffolding...");

    if let Some(path) = get_app_dir() {
        bail!(format!(
            "Already inside cheesecake app: {}",
            path.to_str().unwrap()
        ));
    }

    let outdir = env::current_dir()?.join(name);

    let command_thread = thread::spawn(move || -> anyhow::Result<()> {
        Repository::clone(REPO_URL, outdir.clone())?;

        fs::remove_dir_all(outdir.join(".git"))?;
        Repository::init_opts(
            outdir,
            RepositoryInitOptions::new().initial_head("main"),
        )?;

        Ok(())
    });

    let running = Arc::new(AtomicBool::new(true));
    let running_cloned = running.clone();
    let spinner_thread = thread::spawn(move || {
        while running_cloned.load(Ordering::SeqCst) {
            spinner.tick();
            thread::sleep(Duration::from_millis(100))
        }
    });

    let res = command_thread.join().unwrap();
    running.store(false, Ordering::SeqCst);
    spinner_thread.join().unwrap();

    res
}

pub fn setup_app() -> anyhow::Result<()> {
    normalize_dir("mkdir")?
        .args(["-p", "storage/db"]).status()?;
    normalize_dir("touch")?
        .arg("storage/db/db.sqlite").status()?;
    normalize_dir("cp")?
        .args([".env.example", ".env.local"]).status()?;

    Ok(())
}

pub fn clean_app() -> anyhow::Result<()> {
    normalize_dir("cargo")?.arg("clean").status()?;
    normalize_dir("rm")?
        .args(["-r", "target", "dist"])
        .status()?;

    Ok(())
}

pub fn build_app() -> anyhow::Result<()> {
    todo!();
}

pub fn test_app() -> anyhow::Result<()> {
    normalize_dir("cargo")?.arg("test").status()?;

    Ok(())
}

pub fn check_app() -> anyhow::Result<()> {
    normalize_dir("cargo")?.arg("check").status()?;

    Ok(())
}

pub fn lint_app() -> anyhow::Result<()> {
    normalize_dir("cargo")?.arg("clippy").status()?;

    Ok(())
}

pub fn run_dev() -> anyhow::Result<()> {
    normalize_dir("cargo")?
        .args(["run", "-p", "bin"])
        .env("RUST_BACKTRACE", "1")
        .status()?;

    Ok(())
}

pub fn run_release() -> anyhow::Result<()> {
    normalize_dir("target/release/cheesecake-app")?
        .env("RUST_BACKTRACE", "1")
        .status()?;

    Ok(())
}

pub fn run_task(name: &str) -> anyhow::Result<()> {
    normalize_dir("sh")?
        .args([
            "-c",
            get_task(name).ok_or(anyhow!("Task not found"))?.as_str(),
        ])
        .status()?;

    Ok(())
}
