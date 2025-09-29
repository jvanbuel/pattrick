use std::error::Error;

use reqwest::{Method, StatusCode};

use crate::{PatTokenDeleteRequest, PatTokenManager, AZURE_DEVOPS_PAT_URL, crud::error::DEVOPS_ERROR_MESSAGE};

impl PatTokenManager {
    /// Delete a PAT token
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::{PatTokenManager, PatTokenDeleteRequest};
    /// use pattrick::azure::get_ad_token_for_devops;
    ///
    /// # tokio_test::block_on(async {
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops(1).await?);
    ///
    /// pat_manager.delete_pat_token(
    ///    PatTokenDeleteRequest {
    ///     authorization_id: String::from("12345678-1234-1234-1234-123456789012")
    ///  }
    /// ).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())});
    /// ```
    pub async fn delete_pat_token(
        &self,
        delete_request: PatTokenDeleteRequest,
    ) -> Result<StatusCode, Box<dyn Error>> {
        let response = self
            .base_request(Method::DELETE, AZURE_DEVOPS_PAT_URL)
            .query(&[("authorizationId", &delete_request.authorization_id)])
            .send()
            .await?;

        log::debug!("{:#?}", response);
        match response.error_for_status() {
            Ok(response) => {
                Ok(response.status())
            }
            Err(e) => {
                log::debug!("Error: {:#?}", e);
                if let Some(status) = e.status()
                    && status.is_client_error() {
                        return Err::<StatusCode, Box<dyn Error>>(
                        DEVOPS_ERROR_MESSAGE.into(),
                        );
                    }
                Err::<StatusCode, Box<dyn Error>>(Box::new(e))
            }
        }
    }
}
