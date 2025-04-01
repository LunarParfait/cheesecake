use crate::helpers::get_app_dir;
use anyhow::bail;
use git2::{Repository, RepositoryInitOptions};
use indicatif::ProgressBar;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{env, fs, thread};

pub const REPO_URL: &'static str =
    "https://github.com/LunarParfait/cheesecake-base.git";

pub fn new_application(name: String) -> anyhow::Result<()> {
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
