use std::error::Error;

use reqwest::{Method, StatusCode};

use crate::{PatTokenDeleteRequest, PatTokenManager, AZURE_DEVOPS_PAT_URL};

impl PatTokenManager {
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
