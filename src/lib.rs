use std::error::Error;
use std::process::Command;

mod model;
use async_recursion::async_recursion;
use azure_core::auth::TokenResponse;
use azure_identity::token_credentials::DefaultAzureCredential;
use azure_identity::token_credentials::DefaultAzureCredentialError;
use azure_identity::token_credentials::TokenCredential;
use model::github::GitHubRelease;
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
use reqwest::StatusCode;

const AZURE_DEVOPS_PAT_URL: &str = "https://vssps.dev.azure.com/imec-int/_apis/tokens/pats";
const API_VERSION: &str = "7.1-preview.1";
const DEVOPS_RESOURCE: &str = "499b84ac-1321-427f-aa17-267ca6975798";

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
        let mut pat_tokens: Vec<PatToken> = Vec::new();
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .query(&[("displayFilterOption", &list_request.display_filter_option)])
            .send()
            .await?;

        let mut lt_response = response.json::<ListTokenResponse>().await?;

        pat_tokens.append(&mut lt_response.pat_tokens);

        while let Some(token) = &lt_response.continuation_token {
            if token.is_empty() {
                return Ok(pat_tokens);
            }
            let response = self
                .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
                .query(&[("displayFilterOption", &list_request.display_filter_option)])
                .query(&[("continuationToken", token)])
                .send()
                .await?;

            lt_response = response.json::<ListTokenResponse>().await?;
            pat_tokens.append(&mut lt_response.pat_tokens);
        }

        Ok(lt_response.pat_tokens)
    }

    pub async fn create_pat_token(
        &self,
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
        &self,
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
        &self,
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

    pub async fn get_pat_token_by_name(
        &self,
        name: &str,
    ) -> Result<Option<PatToken>, Box<dyn Error>> {
        let list_request = PatTokenListRequest {
            display_filter_option: DisplayFilterOption::All,
        };
        let pat_tokens = self.list_pat_tokens(&list_request).await?;
        Ok(pat_tokens
            .into_iter()
            .find(|pat_token| pat_token.display_name == name))
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
