use std::error::Error;

use reqwest::Method;

use crate::{
    PatToken, PatTokenCreateRequest, PatTokenManager, PatTokenResult, AZURE_DEVOPS_PAT_URL,
};

impl PatTokenManager {
    /// Create a new PAT token
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pattrick::{PatTokenManager, PatTokenCreateRequest};
    /// use pattrick_clap::Scope;
    /// use pattrick::azure::get_ad_token_for_devops;
    /// use chrono::{Utc, Duration};
    ///
    /// # tokio_test::block_on(async {
    /// let pat_manager = PatTokenManager::new(get_ad_token_for_devops().await?);
    ///
    /// let pat_token = pat_manager.create_pat_token(
    ///    PatTokenCreateRequest {
    ///      all_orgs: false,
    ///      display_name: String::from("awesome-pat"),
    ///      scope: vec![Scope::Build],
    ///      valid_to: (Utc::now() + Duration::seconds(30)),
    ///  }
    /// ).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())});
    ///
    pub async fn create_pat_token(
        &self,
        create_request: PatTokenCreateRequest,
    ) -> Result<PatToken, Box<dyn Error>> {
        let response = self
            .base_request(Method::POST, AZURE_DEVOPS_PAT_URL)
            .json(&create_request)
            .send()
            .await?;

        log::debug!("Response: {:#?}", response);
        let ct_result = response.json::<PatTokenResult>().await?;
        Ok(ct_result.pat_token)
    }
}
