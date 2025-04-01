use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum MigrateCommand {
    /// Generates new migration
    Generate,
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
    /// Resets database
    Reset,
}
