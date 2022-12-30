use std::error::Error;

use reqwest::Method;

use crate::{
    PatToken, PatTokenCreateRequest, PatTokenManager, PatTokenResult, AZURE_DEVOPS_PAT_URL,
};

impl PatTokenManager {
    pub async fn create_pat_token(
        &self,
        create_request: &PatTokenCreateRequest,
    ) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::POST, AZURE_DEVOPS_PAT_URL)
            .json(create_request)
            .send()
            .await?;
            
        log::debug!("Response: {:#?}", response);
        let ct_result = response.json::<PatTokenResult>().await?;
        Ok(ct_result.pat_token)
    }
}
