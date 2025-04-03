use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ViewCommand {
    /// Creates new view
    Create { path: String },
    /// Deletes view
    Delete { path: String },
}
