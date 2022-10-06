use clap::{Parser, Subcommand};

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
    Create(CreateOpts),
    /// List all PAT tokens
    List(ListOpts),
    /// Show contents of a PAT token
    Show(ShowOpts),
    /// Delete a PAT token
    Delete(DeleteOpts),
}
#[derive(clap::Parser)]
pub struct CreateOpts {
    pub name: Option<String>,
    pub lifetime: i32,
}

#[derive(clap::Parser)]
pub struct ListOpts {
    pub all: bool,
}

#[derive(clap::Parser)]
pub struct DeleteOpts {
    pub id: String,
    pub all: bool,
}

#[derive(clap::Parser)]
pub struct ShowOpts {
    pub id: String,
    pub out: Output,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Output {
    StdOut,
    File,
    Env,
}
