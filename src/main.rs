use chrono::Utc;
use clap::Parser;
use output::print_as_table;
use output::write_to_netrc;
use pattrick::DisplayFilterOption;
use pattrick::PatTokenDeleteRequest;
use pattrick::PatTokenGetRequest;
use pattrick::PatTokenListRequest;
use pattrick_clap as args;
use reqwest::{Client, StatusCode};
use std::error::Error;

mod output;
use pattrick::{get_ad_token_for_devops, PatTokenCreateRequest, PatTokenManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = args::Cli::parse();

    let token_manager = PatTokenManager {
        ad_token: get_ad_token_for_devops().await?.token.secret().to_string(),
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
                    write_to_netrc(pat_token)?;
                }
            }
        }
        Some(args::Commands::List(list_opts)) => {
            let list_request = match list_opts.all {
                true => PatTokenListRequest {
                    display_filter_option: DisplayFilterOption::All,
                },
                false => PatTokenListRequest {
                    display_filter_option: DisplayFilterOption::Active,
                },
            };
            let pat_tokens = token_manager.list_pat_tokens(&list_request).await?;

            print_as_table(pat_tokens);
        }
        Some(args::Commands::Get(get_opts)) => {
            let get_request = PatTokenGetRequest {
                authorization_id: get_opts.id.clone(),
            };
            let pat_token = token_manager.get_pat_token(&get_request).await?;
            print_as_table(vec![pat_token]);
        }
        Some(args::Commands::Delete(delete_opts)) => {
            let delete_request = PatTokenDeleteRequest {
                authorization_id: delete_opts.id.clone(),
            };
            let status = &token_manager.delete_pat_token(&delete_request).await?;

            match status {
                &StatusCode::NO_CONTENT => {
                    let id = &delete_opts.id;
                    let check_mark = emoji::symbols::other_symbol::CHECK_MARK_BUTTON.glyph;
                    println!("{check_mark} Successfully deleted PAT token with id: {id}");
                }
                _ => println!("Error deleting token: {status}"),
            }
        }
        _ => {
            println!("other");
        }
    }

    Ok(())
}
