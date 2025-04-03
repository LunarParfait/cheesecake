use self::controller::ControllerCommand;
use self::helpers::get_app_dir;
use self::lifecycle::{
    build_app, check_app, clean_app, lint_app, new_app, run_dev, run_release,
    run_task, setup_app, test_app,
};
use self::model::ModelCommand;
use self::view::ViewCommand;
use crate::migration::MigrateCommand;
use clap::{Parser, Subcommand};

pub mod controller;
pub mod helpers;
pub mod lifecycle;
pub mod migration;
pub mod model;
pub mod view;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates new application in a new directory
    New { name: String },
    /// Sets up application
    Setup,
    /// Cleans application's temp files
    Clean,
    /// Builds application
    Build,
    /// Runs application tests
    Test,
    /// Runs application compilation checks
    Check,
    /// Runs application lint checks
    Lint,
    /// Application running commands
    Run {
        #[command(subcommand)]
        command: RunCommand,
    },
    // TODO: implement these
    // /// Migration related commands
    // Migration {
    //     #[command(subcommand)]
    //     command: MigrateCommand,
    // },
    // /// Model related commands
    // Model {
    //     #[command(subcommand)]
    //     command: ModelCommand,
    // },
    // /// View related commands
    // View {
    //     #[command(subcommand)]
    //     command: ViewCommand,
    // },
    // /// Controller related commands
    // Controller {
    //     #[command(subcommand)]
    //     command: ControllerCommand,
    // },

    /// Gets current app directory
    Dir,
}

#[derive(Debug, Subcommand)]
pub enum RunCommand {
    /// Runs application in dev mode
    Dev,
    /// Runs application in release mode
    Release,
    /// Runs lifecycle task
    Task { name: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::New { name } => new_app(name),
        Command::Setup => setup_app(),
        Command::Clean => clean_app(),
        Command::Build => build_app(),
        Command::Test => test_app(),
        Command::Check => check_app(),
        Command::Lint => lint_app(),
        Command::Run { command } => match command {
            RunCommand::Dev => run_dev(),
            RunCommand::Release => run_release(),
            RunCommand::Task { name } => run_task(name.as_str()),
        },
        Command::Dir => {
            println!("{}", get_app_dir()?.to_str().unwrap());
            Ok(())
        }
    }
}
