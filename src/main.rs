use std::error::Error;
mod args;
use clap::Parser;
mod pat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = args::Cli::parse();

    let res = pat::get_ad_token_for_devops().await?;

    let token_manager = pat::PatTokenManager {
        ad_token: res.token.secret().to_string(),
    };

    let pat_tokens = token_manager.list_pat_tokens();

    println!("{:?}", pat_tokens);

    match &cli.command {
        Some(args::Commands::List(args::ListOpts { all: true })) => {
            println!("patrick List")
        }
        _ => {
            println!("other");
        }
    }

    Ok(())
}
