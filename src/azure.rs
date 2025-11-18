use std::{
    env,
    error::Error,
    fmt::{Display, Formatter},
    process::Command,
};

use async_recursion::async_recursion;
use azure_core::credentials::TokenCredential;
use azure_identity::DeveloperToolsCredential;
use reqwest::Client;

use crate::model::account::AccountsResponse;
use crate::model::profile::Profile;

const DEVOPS_RESOURCE: &str = "499b84ac-1321-427f-aa17-267ca6975798/.default";
const PROFILE_API_URL: &str = "https://app.vssps.visualstudio.com/_apis/profile/profiles/me";
const ACCOUNTS_API_URL: &str = "https://app.vssps.visualstudio.com/_apis/accounts";
const ACCOUNTS_API_VERSION: &str = "7.1";
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
    let res = DeveloperToolsCredential::new(None)?
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

/// Get the user profile from Azure DevOps
///
/// # Example
///
/// ```rust,no_run
/// use pattrick::azure::{get_ad_token_for_devops, get_user_profile};
///
/// # tokio_test::block_on(async {
/// let token = get_ad_token_for_devops(1).await?;
/// let profile = get_user_profile(&token).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())});
/// ```
pub async fn get_user_profile(token: &AzureADToken) -> Result<Profile, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(PROFILE_API_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .query(&[("api-version", ACCOUNTS_API_VERSION)])
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get user profile: {}", response.status()).into());
    }

    let profile = response.json::<Profile>().await?;
    Ok(profile)
}

/// List all Azure DevOps organizations for the authenticated user
///
/// # Example
///
/// ```rust,no_run
/// use pattrick::azure::{get_ad_token_for_devops, list_organizations};
///
/// # tokio_test::block_on(async {
/// let token = get_ad_token_for_devops(1).await?;
/// let orgs = list_organizations(&token).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())});
/// ```
pub async fn list_organizations(token: &AzureADToken) -> Result<Vec<String>, Box<dyn Error>> {
    // First, get the user profile to obtain the user ID
    let profile = get_user_profile(token).await?;

    let client = Client::new();
    let response = client
        .get(ACCOUNTS_API_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .query(&[
            ("memberId", profile.id.as_str()),
            ("api-version", ACCOUNTS_API_VERSION),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to list organizations: {}", response.status()).into());
    }

    let accounts_response = response.json::<AccountsResponse>().await?;
    Ok(accounts_response
        .value
        .into_iter()
        .map(|account| account.account_name)
        .collect())
}

/// Determine the organization to use based on environment variable or available organizations
///
/// # Logic
/// 1. List all organizations the user has access to
/// 2. Check for DEVOPS_ORGANIZATION environment variable
/// 3. If set, verify it's in the user's accessible organizations
/// 4. If not set and only one organization available, use it
/// 5. If not set and multiple organizations available, return an error
///
/// # Example
///
/// ```rust,no_run
/// use pattrick::azure::{get_ad_token_for_devops, get_organization};
///
/// # tokio_test::block_on(async {
/// let token = get_ad_token_for_devops(1).await?;
/// let org = get_organization(&token).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())});
/// ```
pub async fn get_organization(token: &AzureADToken) -> Result<String, Box<dyn Error>> {
    // List organizations that the user has access to
    let orgs = list_organizations(token).await?;

    if orgs.is_empty() {
        return Err("No Azure DevOps organizations found for the authenticated user".into());
    }

    // Check for environment variable
    if let Ok(org) = env::var("DEVOPS_ORGANIZATION")
        && !org.is_empty()
    {
        // Verify that the organization from env var is in the user's accessible organizations
        if orgs.contains(&org) {
            return Ok(org);
        } else {
            return Err(format!(
                    "Organization '{}' from DEVOPS_ORGANIZATION environment variable is not accessible to the authenticated user. Available organizations: {}",
                    org,
                    orgs.join(", ")
                ).into());
        }
    }

    // If no env var, use the single organization or return an error for multiple
    match orgs.len() {
        0 => Err("No Azure DevOps organizations found for the authenticated user".into()),
        1 => Ok(orgs[0].clone()),
        _ => Err(format!(
            "Multiple Azure DevOps organizations found: {}. Please set the DEVOPS_ORGANIZATION environment variable to specify which one to use.",
            orgs.join(", ")
        ).into()),
    }
}
