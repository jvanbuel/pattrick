use std::error::Error;

use reqwest::Method;

use crate::{
    ListTokenResponse, PatToken, PatTokenListRequest, PatTokenManager, AZURE_DEVOPS_PAT_URL, crud::error::DEVOPS_ERROR_MESSAGE,
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
    /// # tokio_test::block_on(async {
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops(1).await?);
    ///
    /// let pat_tokens = pat_manager.list_pat_tokens(
    ///     PatTokenListRequest {
    ///         display_filter_option: DisplayFilterOption::All
    ///     }
    /// ).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())});
    /// ```
    pub async fn list_pat_tokens(
        &self,
        list_request: PatTokenListRequest,
    ) -> Result<Vec<PatToken>, Box<dyn Error>> {
        let mut pat_tokens: Vec<PatToken> = Vec::new();
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .query(&[("displayFilterOption", &list_request.display_filter_option)])
            .send()
            .await?;

        log::debug!("Response: {:#?}", response);
        match response.error_for_status() {
            Ok(response) => {
                log::debug!("{:#?}", response);
                let mut lt_result = response.json::<ListTokenResponse>().await?;
                pat_tokens.append(&mut lt_result.pat_tokens);

                while let Some(token) = &lt_result.continuation_token {
                    if token.is_empty() {
                        return Ok(pat_tokens);
                    }
                    let response = self
                        .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
                        .query(&[("displayFilterOption", &list_request.display_filter_option)])
                        .query(&[("continuationToken", token)])
                        .send()
                        .await?;

                    lt_result = response.json::<ListTokenResponse>().await?;
                    pat_tokens.append(&mut lt_result.pat_tokens);
                    }

                Ok(pat_tokens)
            }
            Err(e) => {
                if let Some(status) = e.status() {
                    if status.is_client_error() {
                        return Err::<Vec<PatToken>, Box<dyn Error>>(DEVOPS_ERROR_MESSAGE.into())
                    }
                }
                Err::<Vec<PatToken>, Box<dyn Error>>(Box::new(e))
            }
        }



    }
}
