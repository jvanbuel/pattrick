use std::error::Error;

use reqwest::{Method, StatusCode};

use crate::{PatTokenDeleteRequest, PatTokenManager, AZURE_DEVOPS_PAT_URL};

impl PatTokenManager {
    /// Delete a PAT token
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::{PatTokenManager, PatTokenDeleteRequest};
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops().await?);
    ///
    /// pat_manager.delete_pat_token(
    ///    PatTokenDeleteRequest {
    ///     authorization_id: "12345678-1234-1234-1234-123456789012"
    ///  }
    /// ).await?;
    /// ```
    pub async fn delete_pat_token(
        &self,
        delete_request: &PatTokenDeleteRequest,
    ) -> Result<StatusCode, Box<dyn Error>> {
        let response = self
            .base_request(Method::DELETE, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", &delete_request.authorization_id)])
            .send()
            .await?;

        log::debug!("Response: {:#?}", response);
        Ok(response.status())
    }
}
