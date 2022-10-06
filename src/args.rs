use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
#[clap(author="Jan Vanbuel", version="0.1.0", about="CLI to manage Azure DevOps Personal Access Tokens (PAT)", long_about = None)]
#[clap(propagate_version = true, arg_required_else_help = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new PAT token
    Create { name: Option<String> },
    /// List all PAT tokens
    List { all: bool },
    /// Show contents of a PAT token
    Show { id: String },
    /// Delete a PAT token
    Delete { all: bool },
}
