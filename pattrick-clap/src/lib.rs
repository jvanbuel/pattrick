use clap::{Parser, Subcommand, ValueEnum};
#[derive(Parser)]
#[clap(author, version, about="CLI to manage Azure DevOps Personal Access Tokens (PAT)", long_about = None)]
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
    Get(GetOpts),
    /// Delete a PAT token
    Delete(DeleteOpts),
}
#[derive(Parser)]
pub struct CreateOpts {
    #[arg(help = "Display name of the PAT token")]
    pub name: Option<String>,
    #[arg(
        long,
        short,
        default_value = "120",
        help = "Number of seconds the token should be valid for"
    )]
    pub lifetime: i64,
    #[arg(
        long,
        short,
        default_value = "vso.packaging",
        help = "Scope of the token"
    )]
    pub scope: String,
    #[arg(short, long, value_enum, default_value_t = Output::StdOut, help = "Output format of the token: print to stdout, write to dotenv or netrc")]
    pub out: Output,
}

#[derive(Parser)]
pub struct ListOpts {
    #[arg(
        default_value_t = false,
        required = false,
        short,
        long,
        help = "Show all tokens, including expired ones"
    )]
    pub all: bool,
}

#[derive(Parser)]
pub struct DeleteOpts {
    pub id: String,
    #[arg(
        default_value_t = false,
        required = false,
        short,
        long,
        help = "Delete all tokens, including expired ones"
    )]
    pub all: bool,
}

#[derive(Parser)]
pub struct GetOpts {
    pub id: String,
    #[arg(short, long, value_enum, default_value_t = Output::StdOut)]
    pub out: Output,
}

#[derive(ValueEnum, Clone)]
pub enum Output {
    StdOut,
    DotNetrc,
    DotEnv,
}
