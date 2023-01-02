use clap::{Parser, Subcommand, ValueEnum};
use strum_macros::Display;

#[derive(Parser)]
#[clap(author, about="CLI to manage Azure DevOps Personal Access Tokens (PAT)", long_about = None)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
    #[clap(long, help = "Print version information")]
    pub version: bool,
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
    /// Update Pattrick
    Update,
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
        value_enum,
        default_values_t = vec![Scope::Packaging],
        help = "Scope of the token"
    )]
    pub scope: Vec<Scope>,
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
    #[arg(default_value_t = String::from(""), required = false, short, long, help = "Name of the token to delete")]
    pub name: String,
    #[arg(default_value_t = String::from(""), required = false, short, long, help = "Id of the token to delete")]
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

#[derive(ValueEnum, Clone, Debug, Default, PartialEq, Eq, Display)]
pub enum Scope {
    FullAccess,
    AgentPools,
    AgentPoolsManage,
    Build,
    BuildExecute,
    Code,
    CodeWrite,
    CodeManage,
    CodeFull,
    Dashboards,
    DashboardsManage,
    DropWrite,
    Extension,
    ExtensionManage,
    Governance,
    Graph,
    GraphManage,
    Notification,
    NotificationDiagnostics,
    #[default]
    Packaging,
    PackagingManage,
    PackagingWrite,
    Profile,
    Project,
    ProjectManage,
    Release,
    ReleaseExecute,
    Security,
    SecurityManage,
    Test,
    TestWrite,
    Work,
    WorkWrite,
    Wiki,
    WikiWrite,
}
