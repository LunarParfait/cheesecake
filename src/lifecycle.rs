use crate::helpers::{
    files_in_dir_recursive, get_app_dir, get_task, normalize_dir,
};
use anyhow::bail;
use git2::build::RepoBuilder;
use git2::{Repository, RepositoryInitOptions};
use indicatif::ProgressBar;
use lightningcss::stylesheet::{
    MinifyOptions, ParserOptions, PrinterOptions, StyleSheet,
};
use minify_html::Cfg;
use minify_js::{Session, TopLevelMode};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;
use std::{env, fs};

pub const REPO_URL: &'static str =
    "https://github.com/LunarParfait/cheesecake-presets.git";
pub const MAJOR_VERSION: &'static str = env!("CARGO_PKG_VERSION_MAJOR");

pub fn new_app(name: String) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Downloading scaffolding...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    if let Ok(path) = get_app_dir() {
        bail!(format!(
            "Already inside cheesecake app: {}",
            path.to_str().unwrap()
        ));
    }

    let outdir = env::current_dir()?.join(name);

    RepoBuilder::new()
        .branch(&format!("base-v{MAJOR_VERSION}"))
        .clone(REPO_URL, &outdir)?;

    fs::remove_dir_all(outdir.join(".git"))?;
    Repository::init_opts(
        outdir,
        RepositoryInitOptions::new().initial_head("main"),
    )?;

    spinner.finish_with_message("Done!");

    Ok(())
}

pub fn setup_app() -> anyhow::Result<()> {
    normalize_dir("mkdir")?
        .args(["-p", "storage/db", "dist"])
        .status()?;
    normalize_dir("touch")?
        .arg("storage/db/db.sqlite")
        .status()?;
    normalize_dir("cp")?
        .args([".env.example", ".env.local"])
        .status()?;

    Ok(())
}

pub fn clean_app() -> anyhow::Result<()> {
    normalize_dir("cargo")?.arg("clean").status()?;
    normalize_dir("rm")?.args(["-r", "dist"]).status()?;

    Ok(())
}

pub fn build_app() -> anyhow::Result<()> {
    normalize_dir("mkdir")?
        .args(["-p", "dist", "dist/static", "dist/templates"])
        .status()?;

    let resources_dir = get_app_dir()?.join("resources");
    let static_dir = resources_dir.join("static");
    let templates_dir = resources_dir.join("templates");

    let dist_dir = get_app_dir()?.join("dist");
    let static_dist_dir = dist_dir.join("static");
    let templates_dist_dir = dist_dir.join("templates");

    let mut html_opts = Cfg::new();
    html_opts.minify_js = true;
    html_opts.minify_css = true;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Minifying js/css and copying static assets...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    for entry in files_in_dir_recursive(static_dir.clone())? {
        let path = entry.path();
        let relpath = path.strip_prefix(static_dir.clone())?;

        match path.extension().map(|x| x.to_str().unwrap()) {
            Some("js") => {
                let mut contents = Vec::new();
                File::open(path.clone())?.read_to_end(&mut contents)?;

                let session = Session::new();
                let mut out = Vec::new();
                minify_js::minify(
                    &session,
                    TopLevelMode::Global,
                    &contents,
                    &mut out,
                )
                .unwrap();

                File::create(static_dist_dir.join(relpath))?.write_all(&out)?;
            }
            Some("css") => {
                let mut contents = Vec::new();
                File::open(entry.path())?.read_to_end(&mut contents)?;
                let contents = String::from_utf8(contents)?;

                let mut stylesheet =
                    StyleSheet::parse(&contents, ParserOptions::default())
                        .unwrap();

                stylesheet.minify(MinifyOptions::default())?;

                let mut opts = PrinterOptions::default();
                opts.minify = true;

                let out = stylesheet.to_css(opts)?;
                File::create(static_dist_dir.join(relpath))?
                    .write_all(out.code.as_bytes())?;
            }
            _ => {
                fs::copy(path.clone(), static_dist_dir.join(relpath))?;
            }
        }
    }

    spinner.set_message("Minifying and moving template files");

    for entry in files_in_dir_recursive(templates_dir.clone())? {
        let path = entry.path();
        let relpath = path.strip_prefix(templates_dir.clone())?;

        let mut contents = Vec::new();
        File::open(path.clone())?.read_to_end(&mut contents)?;

        let out = minify_html::minify(&contents, &html_opts);
        File::create(templates_dist_dir.join(relpath))?.write_all(&out)?;
    }

    spinner.finish_and_clear();

    normalize_dir("cargo")?
        .args(["build", "--release"])
        .status()?;

    Ok(())
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
        .args(["-c", get_task(name)?.as_str()])
        .status()?;

    Ok(())
}
