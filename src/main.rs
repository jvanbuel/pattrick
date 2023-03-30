use chrono::{Duration, Utc};
use clap::{crate_version, Parser};
use output::{print_as_table, write_to_dotenv, write_to_netrc};
use pattrick::{
    DisplayFilterOption, PatTokenDeleteRequest, PatTokenGetRequest, PatTokenListRequest,
};
use pattrick_clap as args;
use reqwest::StatusCode;
use std::error::Error;

mod output;
use pattrick::{azure::get_ad_token_for_devops, PatTokenCreateRequest, PatTokenManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = args::Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    if cli.version {
        println!("pattrick v{}", crate_version!());
        return Ok(());
    }

    let token_manager = PatTokenManager::new(get_ad_token_for_devops(1).await?);

    match &cli.command {
        Some(args::Commands::Create(create_opts)) => {
            let create_request = PatTokenCreateRequest {
                all_orgs: true,
                display_name: create_opts
                    .name
                    .clone()
                    .unwrap_or_else(|| petname::petname(2, "-")),
                scope: create_opts.scope.clone(),
                valid_to: (Utc::now() + Duration::seconds(create_opts.lifetime)),
            };
            log::info!("Creating PAT token with request: {:?}", create_request);
            let pat_token = token_manager.create_pat_token(create_request).await?;

            match create_opts.out {
                args::Output::StdOut => {
                    print_as_table(vec![pat_token], true);
                }
                args::Output::DotEnv => {
                    write_to_dotenv(pat_token)?;
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
            log::info!("Listing PAT tokens with request: {:?}", list_request);
            let pat_tokens = token_manager.list_pat_tokens(list_request).await?;

            print_as_table(pat_tokens, false);
        }
        Some(args::Commands::Get(get_opts)) => {
            let get_request = PatTokenGetRequest {
                authorization_id: get_opts.id.clone(),
            };
            let pat_token = token_manager.get_pat_token(get_request).await?;
            print_as_table(vec![pat_token], false);
        }
        Some(args::Commands::Delete(delete_opts)) => {
            if delete_opts.id.is_empty() && delete_opts.name.is_empty() {
                return Err("Either id or name must be provided".into());
            }
            let mut authorization_id = "".to_string();
            if !delete_opts.name.is_empty() {
                let token_result = &token_manager
                    .get_pat_token_by_name(&delete_opts.name)
                    .await?;
                match token_result {
                    Some(token) => {
                        authorization_id = token.id.clone();
                    }
                    None => {
                        return Err("No PAT token found with that name".into());
                    }
                }
            }
            if !delete_opts.id.is_empty() {
                authorization_id = delete_opts.id.clone();
            }
            let delete_request = PatTokenDeleteRequest {
                authorization_id: authorization_id.clone(),
            };

            log::info!("Deleting PAT token with request: {:?}", delete_request);
            let status = &token_manager.delete_pat_token(delete_request).await?;

            match status {
                &StatusCode::NO_CONTENT => {
                    println!("✅ Successfully deleted PAT token with id: {authorization_id}");
                }
                _ => println!("Error deleting token: {status}"),
            }
        }
        Some(args::Commands::Update) => {
            log::info!("Checking for updates...");
            let current_version = crate_version!();
            let latest_version = &token_manager.get_latest_version().await?;
            if latest_version.contains(current_version) {
                println!("You are running the latest version of pattrick (v{current_version})");
                return Ok(());
            }

            log::info!("Found new version: {latest_version}. Updating...");
            let status = tokio::task::spawn_blocking(|| {
                self_update::backends::github::Update::configure()
                    .repo_owner("jvanbuel")
                    .repo_name("pattrick")
                    .bin_name("pattrick")
                    .show_download_progress(true)
                    .current_version(crate_version!())
                    .build()
                    .unwrap()
                    .update()
            })
            .await?;

            println!("Status of update: {status:?}");
        }
        _ => {}
    }

    Ok(())
}
