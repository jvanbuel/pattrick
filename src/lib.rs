use std::error::Error;

use azure_core::auth::TokenResponse;
use azure_identity::token_credentials::AzureCliCredential;
use azure_identity::token_credentials::TokenCredential;
use chrono::{DateTime, Utc};
use reqwest::header;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::Method;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use tabled::Tabled;

const AZURE_DEVOPS_PAT_URL: &str = "https://vssps.dev.azure.com/imec-int/_apis/tokens/pats";
const API_VERSION: &str = "7.1-preview.1";

#[derive(Tabled, Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatToken {
    #[serde(rename = "authorizationId")]
    pub id: String,
    pub display_name: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub scope: String,
    #[tabled(display_with = "display_token")]
    pub token: Option<String>,
    // #[serde(skip)]
    // pub target_accounts: Vec<String>,
}

fn display_token(token: &Option<String>) -> String {
    match token {
        Some(token) => token.to_string(),
        None => "N/A".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListTokenResponse {
    pub continuation_token: Option<String>,
    pub pat_tokens: Vec<PatToken>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenResult {
    pat_token: PatToken,
    pat_token_error: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenCreateRequest {
    pub all_orgs: bool,
    pub display_name: String,
    pub scope: String,
    pub valid_to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenListRequest {
    pub display_filter_option: DisplayFilterOption,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum DisplayFilterOption {
    All,
    #[default]
    Active,
    Expired,
    Revoked,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenGetRequest {
    pub authorization_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenDeleteRequest {
    pub authorization_id: String,
}

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
    pub async fn list_pat_tokens(
        &self,
        list_request: &PatTokenListRequest,
    ) -> Result<Vec<PatToken>, Box<dyn Error>> {
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .query(&[("displayFilterOption", &list_request.display_filter_option)])
            .send()
            .await?;

        let lt_response = response.json::<ListTokenResponse>().await?;

        Ok(lt_response.pat_tokens)
    }

    pub async fn create_pat_token(
        self,
        create_request: &PatTokenCreateRequest,
    ) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::POST, AZURE_DEVOPS_PAT_URL)
            .json(create_request)
            .send()
            .await?;

        let ct_result = response.json::<PatTokenResult>().await?;
        Ok(ct_result.pat_token)
    }

    pub async fn delete_pat_token(
        self,
        delete_request: &PatTokenDeleteRequest,
    ) -> Result<StatusCode, Box<dyn Error>> {
        let response = self
            .base_request(Method::DELETE, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", &delete_request.authorization_id)])
            .send()
            .await?;

        Ok(response.status())
    }

    pub async fn get_pat_token(
        self,
        get_request: &PatTokenGetRequest,
    ) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", &get_request.authorization_id)])
            .send()
            .await?;

        let lt_response = response.json::<PatTokenResult>().await?;

        Ok(lt_response.pat_token)
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

pub async fn get_ad_token_for_devops() -> Result<TokenResponse, Box<dyn Error>> {
    let res = AzureCliCredential
        .get_token("499b84ac-1321-427f-aa17-267ca6975798")
        .await?;
    Ok(res)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GitHubRelease {
    url: String,
    assets_url: String,
    tag_name: String,
    published_at: DateTime<Utc>,
}
