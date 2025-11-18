use serde::{Deserialize, Serialize};

/// Represents an Azure DevOps account/organization
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Account ID
    pub account_id: String,
    /// Account name (organization name)
    pub account_name: String,
    /// Account URI
    pub account_uri: String,
}

/// Response from the Azure DevOps accounts API
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsResponse {
    /// Number of accounts
    pub count: i32,
    /// List of accounts
    pub value: Vec<Account>,
}
