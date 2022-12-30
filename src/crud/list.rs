use std::error::Error;

use reqwest::Method;

use crate::{
    ListTokenResponse, PatToken, PatTokenListRequest, PatTokenManager, AZURE_DEVOPS_PAT_URL,
};

impl PatTokenManager {
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
}
