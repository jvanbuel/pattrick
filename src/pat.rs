use std::error::Error;

use azure_core::auth::TokenResponse;
use azure_identity::token_credentials::AzureCliCredential;
use azure_identity::token_credentials::TokenCredential;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::Method;
use serde::Deserialize;
use serde::Serialize;

use crate::args::ListOpts;

const AZURE_DEVOPS_PAT_URL: &str = "https://vssps.dev.azure.com/imec-int/_apis/tokens/pats";
const API_VERSION: &str = "7.1-preview.1";

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatToken {
    pub display_name: String,
    pub valid_to: String,
    pub scope: String,
    pub target_accounts: Vec<String>,
    pub valid_from: String,
    pub authorization_id: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListTokenResponse {
    pub continuation_token: Option<String>,
    pub pat_tokens: Vec<PatToken>,
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

        println!("{:?}", &lt_response);

        Ok(vec![PatToken::default()])
    }

    pub async fn create_pat_token(self, _valid_to: String) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .client
            .post(AZURE_DEVOPS_PAT_URL)
            .bearer_auth(&self.ad_token)
            .query(&[("api-version", API_VERSION)])
            .send()
            .await?;

        println!("{:?}", response.text().await?);

        Ok(PatToken::default())
    }

    pub async fn delete_pat_token(self, id: String) -> Result<(), Box<dyn Error>> {
        let response = self
            .client
            .delete(&format!("{}/{}", AZURE_DEVOPS_PAT_URL, id))
            .bearer_auth(&self.ad_token)
            .query(&[("api-version", API_VERSION)])
            .send()
            .await?;

        println!("{:?}", response.text().await?);

        Ok(())
    }

    pub async fn show_pat_token(self, id: String) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .client
            .get(&format!("{}/{}", AZURE_DEVOPS_PAT_URL, id))
            .bearer_auth(&self.ad_token)
            .query(&[("api-version", API_VERSION)])
            .send()
            .await?;

        println!("{:?}", response.text().await?);

        Ok(PatToken::default())
    }
}

pub async fn get_ad_token_for_devops() -> Result<TokenResponse, Box<dyn Error>> {
    let res = AzureCliCredential
        .get_token("499b84ac-1321-427f-aa17-267ca6975798")
        .await?;
    Ok(res)
}
