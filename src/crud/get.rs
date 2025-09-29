use std::error::Error;

use crate::crud::error::DEVOPS_ERROR_MESSAGE;
use reqwest::Method;

use crate::{
    AZURE_DEVOPS_PAT_URL, DisplayFilterOption, PatToken, PatTokenGetRequest, PatTokenListRequest,
    PatTokenManager, PatTokenResult,
};

impl PatTokenManager {
    /// Get a PAT token
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::{PatTokenManager, PatTokenGetRequest};
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// # tokio_test::block_on(async {
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops(1).await?);
    ///
    /// let pat_token = pat_manager.get_pat_token(
    ///    PatTokenGetRequest {
    ///       authorization_id: String::from("12345678-1234-1234-1234-123456789012")
    ///   }
    /// ).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())});
    /// ```
    pub async fn get_pat_token(
        &self,
        get_request: PatTokenGetRequest,
    ) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::GET, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", get_request.authorization_id)])
            .send()
            .await?;

        log::debug!("Response: {:#?}", response);
        match response.error_for_status() {
            Ok(response) => {
                log::debug!("{:#?}", response);
                let lt_result = response.json::<PatTokenResult>().await?;
                Ok(lt_result.pat_token)
            }
            Err(e) => {
                log::debug!("Error: {:#?}", e);
                if let Some(status) = e.status()
                    && status.is_client_error()
                {
                    return Err::<PatToken, Box<dyn Error>>(DEVOPS_ERROR_MESSAGE.into());
                }
                Err::<PatToken, Box<dyn Error>>(Box::new(e))
            }
        }
    }

    /// Get a PAT token by name
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::PatTokenManager;
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// # tokio_test::block_on( async {
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops(1).await?);
    ///
    /// let pat_token = pat_manager.get_pat_token_by_name("awesome-pat").await?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())});
    /// ```
    pub async fn get_pat_token_by_name(
        &self,
        name: &str,
    ) -> Result<Option<PatToken>, Box<dyn Error>> {
        let list_request = PatTokenListRequest {
            display_filter_option: DisplayFilterOption::All,
        };
        let pat_tokens = self.list_pat_tokens(list_request).await?;
        Ok(pat_tokens
            .into_iter()
            .find(|pat_token| pat_token.display_name == name))
    }
}
