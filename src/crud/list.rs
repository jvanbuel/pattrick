use std::error::Error;

use reqwest::Method;

use crate::{
    ListTokenResponse, PatToken, PatTokenListRequest, PatTokenManager, AZURE_DEVOPS_PAT_URL,
};

impl PatTokenManager {
    /// List PAT tokens
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::{PatTokenManager, PatTokenListRequest, DisplayFilterOption};
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops().await?);
    ///
    /// let pat_tokens = pat_manager.list_pat_tokens(
    ///     PatTokenListRequest {
    ///         display_filter_option: DisplayFitlerOption::All
    ///     }
    /// ).await?;
    /// ```
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

        log::debug!("Response: {:#?}", response);
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
