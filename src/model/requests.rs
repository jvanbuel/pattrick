use crate::model::token::{scopes_from_string, scopes_to_string, PatToken};
use chrono::{DateTime, Utc};
use pattrick_clap::Scope;
use serde::{Deserialize, Serialize};

/// Response from the Azure DevOps API when listing PAT tokens
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListTokenResponse {
    /// continuation token for the next page of results
    pub continuation_token: Option<String>,
    /// list of PAT tokens
    pub pat_tokens: Vec<PatToken>,
}

/// Result of a PAT token creation request
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenResult {
    /// the created PAT token
    pub pat_token: PatToken,
    /// error message if the creation failed
    pub pat_token_error: String,
}

/// Request to create a PAT token
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenCreateRequest {
    /// is the token valid for all organizations?
    pub all_orgs: bool,
    /// display name of the token
    pub display_name: String,
    #[serde(
        deserialize_with = "scopes_from_string",
        serialize_with = "scopes_to_string"
    )]
    /// scope of the PAT token, e.g. Packaging, Code, Build, ...
    pub scope: Vec<Scope>,
    /// expiration date of the token
    pub valid_to: DateTime<Utc>,
}

/// Request to list a PAT token
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenListRequest {
    /// display filter options for the list (e.g. all, active, expired, revoked)
    pub display_filter_option: DisplayFilterOption,
}

/// Display filter options for listing PAT tokens
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum DisplayFilterOption {
    /// list all PAT tokens
    All,
    /// list only active PAT tokens
    #[default]
    Active,
    /// list only expired PAT tokens
    Expired,
    /// list only revoked PAT tokens
    Revoked,
}

/// Request to get a PAT token
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenGetRequest {
    /// id of the PAT token
    pub authorization_id: String,
}

/// Request to revoke a PAT token
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenDeleteRequest {
    /// id of the PAT token
    pub authorization_id: String,
}
