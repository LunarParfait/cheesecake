use clap::{Args, Subcommand};

#[derive(Debug, Clone, Args)]
pub struct ModelArgs {
    name: String,
    columns: Vec<String>,
    // TODO: add relations
}

#[derive(Debug, Subcommand)]
pub enum ModelCommand {
    /// Creates new model as a new migration
    Create(ModelArgs),
    /// Deletes model as a new migration
    Delete { path: String },
}
