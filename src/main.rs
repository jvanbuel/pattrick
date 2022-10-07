use std::error::Error;
mod args;
use clap::Parser;
use reqwest::Client;
use tabled::{Style, Table};
mod pat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = args::Cli::parse();

    let token_manager = pat::PatTokenManager {
        ad_token: pat::get_ad_token_for_devops()
            .await?
            .token
            .secret()
            .to_string(),
        client: Client::new(),
    };

    match &cli.command {
        Some(args::Commands::Create(create_opts)) => {
            let pat_token = &token_manager
                .create_pat_token(create_opts.lifetime.to_string())
                .await?;
            println!("{:?}", pat_token);
        }
        Some(args::Commands::List(list_opts)) => {
            let pat_tokens = &token_manager.list_pat_tokens(list_opts).await?;

            let mut table = Table::new(pat_tokens);
            table.with(Style::modern());
            println!("{:#^10}", table.to_string());
        }
        Some(args::Commands::Show(show_opts)) => {
            let pat_token = &token_manager
                .show_pat_token(show_opts.id.to_string())
                .await?;
            println!("{:?}", pat_token);
        }
        Some(args::Commands::Delete(delete_opts)) => {
            let pat_token = &token_manager
                .delete_pat_token(delete_opts.id.to_string())
                .await?;
            println!("{:?}", pat_token);
        }
        _ => {
            println!("other");
        }
    }

    Ok(())
}
