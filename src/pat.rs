use std::error::Error;

use azure_core::auth::TokenResponse;
use azure_identity::token_credentials::AzureCliCredential;
use azure_identity::token_credentials::TokenCredential;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::Method;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use tabled::Tabled;

use crate::args::DeleteOpts;
use crate::args::ListOpts;
use crate::args::ShowOpts;

const AZURE_DEVOPS_PAT_URL: &str = "https://vssps.dev.azure.com/imec-int/_apis/tokens/pats";
const API_VERSION: &str = "7.1-preview.1";

#[derive(Tabled, Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatToken {
    #[serde(rename = "authorizationId")]
    pub id: String,
    pub display_name: String,
    pub valid_from: String,
    pub valid_to: String,
    pub scope: String,
    // #[serde(skip)]
    // pub target_accounts: Vec<String>,
    // pub token: Option<String>,
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
        _list_opts: &ListOpts,
    ) -> Result<Vec<PatToken>, Box<dyn Error>> {
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .send()
            .await?;

        let lt_response = response.json::<ListTokenResponse>().await?;

        Ok(lt_response.pat_tokens)
    }

    pub async fn create_pat_token(
        self,
        create_opts: &PatTokenCreateRequest,
    ) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::POST, AZURE_DEVOPS_PAT_URL)
            .json(create_opts)
            .send()
            .await?;

        let ct_result = response.json::<PatTokenResult>().await?;
        Ok(ct_result.pat_token)
    }

    pub async fn delete_pat_token(
        self,
        delete_opts: &DeleteOpts,
    ) -> Result<StatusCode, Box<dyn Error>> {
        let response = self
            .base_request(Method::DELETE, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", &delete_opts.id)])
            .send()
            .await?;

        Ok(response.status())
    }

    pub async fn show_pat_token(self, show_opts: &ShowOpts) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", &show_opts.id)])
            .send()
            .await?;

        let lt_response = response.json::<PatTokenResult>().await?;

        Ok(lt_response.pat_token)
    }
}

pub async fn get_ad_token_for_devops() -> Result<TokenResponse, Box<dyn Error>> {
    let res = AzureCliCredential
        .get_token("499b84ac-1321-427f-aa17-267ca6975798")
        .await?;
    Ok(res)
}
