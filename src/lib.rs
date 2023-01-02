//! # Pattrick
//!
//! `pattrick` is a library to manage personal access tokens (PAT) for Azure DevOps.
//! It allows you to easily create, list, get and delete PAT tokens.
//!
//! # Example
//! ```
//!
//! use pattrick::{PatTokenManager, PatTokenListRequest, DisplayFilterOption};
//! use pattrick::azure::get_ad_token_for_devops;
//!
//! let pat_manager = PatTokenManager::new(get_ad_token_for_devops().await?);
//!
//! let pat_tokens = pat_manager.list_pat_tokens(
//!     PatTokenListRequest {
//!         display_filter_option: DisplayFilterOption::All
//!     }
//! ).await?;
//! ```

#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

use std::error::Error;
/// Azure AD related functions
pub mod azure;
mod crud;
mod model;
use azure::AzureADToken;
use model::release::GitHubRelease;
pub use model::{
    requests::{
        DisplayFilterOption, ListTokenResponse, PatTokenCreateRequest, PatTokenDeleteRequest,
        PatTokenGetRequest, PatTokenListRequest, PatTokenResult,
    },
    token::PatToken,
};
use reqwest::header;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::Method;

const AZURE_DEVOPS_PAT_URL: &str = "https://vssps.dev.azure.com/imec-int/_apis/tokens/pats";
const API_VERSION: &str = "7.1-preview.1";

/// PatTokenManager is a struct that manages the creation, listing, getting and deletion of PAT tokens.
/// It uses the Azure AD token to authenticate with Azure DevOps.
///
/// # Example
///
/// ```rust,no_run
/// use pattrick::PatTokenManager;
/// use pattrick::azure::get_ad_token_for_devops;
/// use reqwest::Client;
///
/// let pat_manager = PatTokenManager {
///     ad_token: get_ad_token_for_devops().await?,
///     client: Client::new(),
/// };
/// ```
pub struct PatTokenManager {
    /// Azure AD token used to authenticate with Azure DevOps
    pub ad_token: AzureADToken,
    /// Reqwest client used to make requests
    pub client: Client,
}

impl PatTokenManager {
    /// Create a new PatTokenManager
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::PatTokenManager;
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops().await?);
    /// ```
    pub fn new(ad_token: AzureADToken) -> Self {
        Self {
            ad_token,
            client: Client::new(),
        }
    }
    fn base_request<U>(&self, method: Method, url: U) -> reqwest::RequestBuilder
    where
        U: IntoUrl,
    {
        self.client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", self.ad_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&[("api-version", API_VERSION)])
    }
    /// Get the lastest released version of the Pattrick CLI from GitHub
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::PatTokenManager;
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops().await?);
    ///
    /// let latest_version = pat_manager.get_latest_version().await?;
    ///
    /// println!("Latest version: {}", latest_version);
    ///
    pub async fn get_latest_version(self) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .request(
                Method::GET,
                "https://api.github.com/repos/jvanbuel/pattrick/releases",
            )
            .header(header::USER_AGENT, "Pattrick")
            .header(header::HOST, "api.github.com")
            .send()
            .await?;

        let gh_response = response.json::<Vec<GitHubRelease>>().await?;
        Ok(gh_response[0].tag_name.clone())
    }
}
