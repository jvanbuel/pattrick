use azure_core::auth::{TokenCredential, TokenResponse};
use azure_identity::token_credentials::AzureCliCredential;
use std::error::Error;
mod args;
use clap::Parser;

#[derive(Debug)]
struct PatToken {
    token: String,
}

struct PatTokenManager {
    ad_token: String,
}

impl PatTokenManager {
    fn list_pat_tokens(self) -> Vec<PatToken> {
        vec![PatToken {
            token: self.ad_token,
        }]
    }
}

async fn get_ad_token_for_devops() -> Result<TokenResponse, Box<dyn Error>> {
    let res = AzureCliCredential
        .get_token("499b84ac-1321-427f-aa17-267ca6975798")
        .await?;
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = args::Cli::parse();

    let sub_id = AzureCliCredential::get_subscription()?;
    println!("Azure cli subscription: {}", sub_id);

    let res = get_ad_token_for_devops().await?;

    let token_manager = PatTokenManager {
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
