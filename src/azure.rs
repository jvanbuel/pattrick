use std::{
    error::Error,
    fmt::{Display, Formatter},
    process::Command,
};

use async_recursion::async_recursion;
use azure_core::credentials::TokenCredential;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};

const DEVOPS_RESOURCE: &str = "499b84ac-1321-427f-aa17-267ca6975798/.default";
/// Azure AD token
///
/// # Example
///
/// ```no_run
/// use pattrick::azure::get_ad_token_for_devops;
/// use pattrick::azure::AzureADToken;
///
/// # tokio_test::block_on(async {
/// let token: AzureADToken = get_ad_token_for_devops(1).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())});
/// ```
#[derive(Debug)]
pub struct AzureADToken(String);

impl Display for AzureADToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Get an Azure AD token for Azure DevOps
///
/// # Example
///
/// ```rust,no_run
/// use pattrick::azure::get_ad_token_for_devops;
///
/// # tokio_test::block_on(async {
/// let token = get_ad_token_for_devops(1).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())});
/// ```
#[async_recursion]
pub async fn get_ad_token_for_devops(tries: i8) -> Result<AzureADToken, Box<dyn Error>> {
    let res = DefaultAzureCredential::with_options(TokenCredentialOptions::default())?
        .get_token(&[DEVOPS_RESOURCE], None)
        .await;
    match res {
        Ok(token_response) => Ok(AzureADToken(token_response.token.secret().to_string())),
        Err(e) => {
            if tries > 0 {
                println!("🔐 No credential available. Trying to log in with az cli...");
                Command::new("az")
                    .args(vec!["login"])
                    .output()
                    .expect("Login failed.");
                get_ad_token_for_devops(tries - 1).await
            } else {
                Err::<AzureADToken, Box<dyn Error>>(Box::new(e))
            }
        }
    }
}
