use std::{error::Error, process::Command};

use async_recursion::async_recursion;
use azure_core::auth::TokenResponse;
use azure_identity::token_credentials::{
    DefaultAzureCredential, DefaultAzureCredentialError, TokenCredential,
};

const DEVOPS_RESOURCE: &str = "499b84ac-1321-427f-aa17-267ca6975798";

#[async_recursion]
pub async fn get_ad_token_for_devops() -> Result<TokenResponse, Box<dyn Error>> {
    let res = DefaultAzureCredential::default()
        .get_token(DEVOPS_RESOURCE)
        .await;
    match res {
        Ok(token) => Ok(token),
        Err(e) => {
            if let DefaultAzureCredentialError::CredentialUnavailable(_) = e {
                println!("🔐 No credential available. Logging in with az cli...");
                Command::new("az")
                    .args(vec!["login"])
                    .output()
                    .expect("Login failed.");
                get_ad_token_for_devops().await
            } else {
                Err::<TokenResponse, Box<dyn Error>>(Box::new(e))
            }
        }
    }
}
