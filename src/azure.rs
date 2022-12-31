use std::{
    error::Error,
    fmt::{Display, Formatter},
    process::Command,
};

use async_recursion::async_recursion;
use azure_identity::token_credentials::{
    DefaultAzureCredential, DefaultAzureCredentialError, TokenCredential,
};

const DEVOPS_RESOURCE: &str = "499b84ac-1321-427f-aa17-267ca6975798";
/// Azure AD token
///
/// # Example
///
/// ```rust,no_run
/// use pattrick::azure::get_ad_token_for_devops;
///
/// let token: AzureADToken = get_ad_token_for_devops().await?;
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
/// let token = get_ad_token_for_devops().await?;
/// ```
#[async_recursion]
pub async fn get_ad_token_for_devops() -> Result<AzureADToken, Box<dyn Error>> {
    let res = DefaultAzureCredential::default()
        .get_token(DEVOPS_RESOURCE)
        .await;
    match res {
        Ok(token_response) => Ok(AzureADToken(token_response.token.secret().to_string())),
        Err(e) => {
            if let DefaultAzureCredentialError::CredentialUnavailable(_) = e {
                println!("🔐 No credential available. Logging in with az cli...");
                Command::new("az")
                    .args(vec!["login"])
                    .output()
                    .expect("Login failed.");
                get_ad_token_for_devops().await
            } else {
                Err::<AzureADToken, Box<dyn Error>>(Box::new(e))
            }
        }
    }
}
