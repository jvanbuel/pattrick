use std::error::Error;

use reqwest::Method;

use crate::{
    DisplayFilterOption, PatToken, PatTokenGetRequest, PatTokenListRequest, PatTokenManager,
    PatTokenResult, AZURE_DEVOPS_PAT_URL,
};

impl PatTokenManager {
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
}
