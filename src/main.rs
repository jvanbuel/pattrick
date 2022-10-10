use std::{error::Error, path::Path};
mod args;
use chrono::Utc;
use clap::Parser;
use pat::PatToken;
use reqwest::{Client, StatusCode};
use tabled::{Style, Table};

use crate::pat::PatTokenCreateRequest;
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
            let create_request = PatTokenCreateRequest {
                all_orgs: true,
                display_name: create_opts
                    .name
                    .clone()
                    .unwrap_or_else(|| petname::petname(2, "-")),
                scope: create_opts.scope.to_string(),
                valid_to: (Utc::now() + chrono::Duration::seconds(create_opts.lifetime))
                    .to_rfc3339(),
            };

            let pat_token = token_manager.create_pat_token(&create_request).await?;

            match create_opts.out {
                args::Output::StdOut => {
                    print_as_table(vec![pat_token]);
                }
                args::Output::DotEnv => {
                    std::fs::write(
                        ".env",
                        format!("{}={}", pat_token.display_name, pat_token.token.unwrap()),
                    )?;
                }
                args::Output::DotNetrc => {
                    let netrc_path = dirs::home_dir()
                        .ok_or("Could not find home directory")?
                        .join(".netrc");
                    let netrc_contents = std::fs::read_to_string(netrc_path)?;
                    let netrc = netrc::Netrc::parse(netrc_contents.as_bytes());

                    
                    println!("{:?}", netrc);
                    // std::fs::write(
                    //     "token.txt",
                    //     format!("{}={}", pat_token.display_name, pat_token.token.unwrap()),
                    // )?;
                }
            }
        }
        Some(args::Commands::List(list_opts)) => {
            let pat_tokens = token_manager.list_pat_tokens(list_opts).await?;

            print_as_table(pat_tokens);
        }
        Some(args::Commands::Show(show_opts)) => {
            let pat_token = token_manager.show_pat_token(show_opts).await?;
            print_as_table(vec![pat_token]);
        }
        Some(args::Commands::Delete(delete_opts)) => {
            let status = &token_manager.delete_pat_token(delete_opts).await?;

            match status {
                &StatusCode::NO_CONTENT => {
                    let id = &delete_opts.id;
                    let check_mark = emoji::symbols::other_symbol::CHECK_MARK_BUTTON.glyph;
                    println!("{check_mark} Successfully deleted PAT token with id: {id}");
                }
                _ => println!("Error deleting token: {:?}", status),
            }
        }
        _ => {
            println!("other");
        }
    }

    Ok(())
}

fn print_as_table(pat_tokens: Vec<PatToken>) {
    let mut table = Table::new(pat_tokens);
    table.with(Style::modern());
    println!("{:#^10}", table.to_string());
}
