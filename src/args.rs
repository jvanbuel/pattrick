use clap::{Parser, Subcommand, ValueEnum};

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
#[derive(Parser)]
pub struct CreateOpts {
    pub name: Option<String>,
    #[arg(default_value = "2022-12-31T23:59:59.9999999")]
    pub lifetime: String,
}

#[derive(Parser)]
pub struct ListOpts {
    pub all: bool,
}

#[derive(Parser)]
pub struct DeleteOpts {
    pub id: String,
    pub all: bool,
}

#[derive(Parser)]
pub struct ShowOpts {
    pub id: String,
    #[arg(value_enum, default_value_t = Output::StdOut)]
    pub out: Output,
}

#[derive(ValueEnum, Clone)]
pub enum Output {
    StdOut,
    File,
    Env,
}
