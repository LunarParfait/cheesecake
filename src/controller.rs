use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ControllerCommand {
    /// Creates new controller
    Create { path: String },
    /// Deletes controller
    Delete { path: String },
}
