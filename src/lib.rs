use std::error::Error;
pub mod azure;
mod crud;
mod model;
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

pub struct PatTokenManager {
    pub ad_token: String,
    pub client: Client,
}

impl PatTokenManager {
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
