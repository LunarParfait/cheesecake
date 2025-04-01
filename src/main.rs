use self::controller::ControllerCommand;
use self::helpers::get_app_dir;
use self::lifecycle::new_application;
use self::model::ModelCommand;
use self::view::ViewCommand;
use crate::migration::MigrateCommand;
use anyhow::anyhow;
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
    /// Migration related commands
    Migration {
        #[command(subcommand)]
        command: MigrateCommand,
    },
    /// Model related commands
    Model {
        #[command(subcommand)]
        command: ModelCommand,
    },
    /// View related commands
    View {
        #[command(subcommand)]
        command: ViewCommand,
    },
    /// Controller related commands
    Controller {
        #[command(subcommand)]
        command: ControllerCommand,
    },
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
        Command::New { name } => new_application(name),
        Command::Clean => todo!(),
        Command::Build => todo!(),
        Command::Test => todo!(),
        Command::Check => todo!(),
        Command::Lint => todo!(),
        Command::Run { command } => todo!(),
        Command::Migration { command } => todo!(),
        Command::Model { command } => todo!(),
        Command::View { command } => todo!(),
        Command::Controller { command } => todo!(),
        Command::Dir => {
            println!(
                "{}",
                get_app_dir()
                    .ok_or(anyhow!("Cheesecake application not found"))
                    .map(|p| p.to_str().unwrap().to_owned())?
            );
            Ok(())
        }
    }
}
