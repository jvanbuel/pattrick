use serde::{Deserialize, Serialize};

/// Represents an Azure DevOps user profile
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// User ID
    pub id: String,
    /// Display name
    pub display_name: Option<String>,
    /// Email address
    pub email_address: Option<String>,
    /// Public alias
    pub public_alias: Option<String>,
}
