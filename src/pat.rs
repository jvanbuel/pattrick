use std::error::Error;

use azure_core::auth::TokenResponse;
use azure_identity::token_credentials::AzureCliCredential;
use azure_identity::token_credentials::TokenCredential;

#[derive(Debug)]
pub struct PatToken {
    pub token: String,
}

pub struct PatTokenManager {
    pub ad_token: String,
}

impl PatTokenManager {
    pub fn list_pat_tokens(self) -> Vec<PatToken> {
        vec![PatToken {
            token: self.ad_token,
        }]
    }
}

pub async fn get_ad_token_for_devops() -> Result<TokenResponse, Box<dyn Error>> {
    let res = AzureCliCredential
        .get_token("499b84ac-1321-427f-aa17-267ca6975798")
        .await?;
    Ok(res)
}
