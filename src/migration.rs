use clap::Subcommand;

use crate::helpers::normalize_dir;

#[derive(Debug, Subcommand)]
pub enum MigrateCommand {
    /// Generates new migration
    Generate {
        /// Migration name
        name: String,
    },
    /// Runs a single migration
    Run,
    /// Runs all pending migrations
    RunAll,
    /// Rollsback a single migration
    Rollback,
    /// Rollsback all migrations
    RollbackAll,
    /// Display migration status
    Status,
}

pub fn handle_command(command: MigrateCommand) -> anyhow::Result<()> {
    match command {
        MigrateCommand::Generate { name } => generate_migration(name),
        MigrateCommand::Run => run_migration(),
        MigrateCommand::RunAll => run_all_migrations(),
        MigrateCommand::Rollback => rollback_migration(),
        MigrateCommand::RollbackAll => rollback_all_migrations(),
        MigrateCommand::Status => display_status(),
    }
}

const MIGRATIONS_DIR: &'static str = "db/migrations";

pub fn generate_migration(name: String) -> anyhow::Result<()> {
    normalize_dir(MIGRATIONS_DIR, "cargo")?
        .args(["run", "--", "generate", name.as_str()])
        .status()?;

    Ok(())
}

pub fn run_migration() -> anyhow::Result<()> {
    normalize_dir(MIGRATIONS_DIR, "cargo")?
        .args(["run", "--", "up", "-n", "1"])
        .status()?;
    Ok(())
}

pub fn run_all_migrations() -> anyhow::Result<()> {
    normalize_dir(MIGRATIONS_DIR, "cargo")?.args(["run", "--", "up"]).status()?;

    Ok(())
}

pub fn rollback_migration() -> anyhow::Result<()> {
    normalize_dir(MIGRATIONS_DIR, "cargo")?
        .args(["run", "--", "down", "-n", "1"])
        .status()?;

    Ok(())
}

pub fn rollback_all_migrations() -> anyhow::Result<()> {
    normalize_dir(MIGRATIONS_DIR, "cargo")?
        .args(["run", "--", "reset"])
        .status()?;

    Ok(())
}

pub fn display_status() -> anyhow::Result<()> {
    normalize_dir(MIGRATIONS_DIR, "cargo")?
        .args(["run", "--", "status"])
        .status()?;

    Ok(())
}
