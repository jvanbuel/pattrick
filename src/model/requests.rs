use crate::model::token::{scopes_from_string, scopes_to_string, PatToken};
use pattrick_clap::Scope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListTokenResponse {
    pub continuation_token: Option<String>,
    pub pat_tokens: Vec<PatToken>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenResult {
    pub pat_token: PatToken,
    pub pat_token_error: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenCreateRequest {
    pub all_orgs: bool,
    pub display_name: String,
    #[serde(
        deserialize_with = "scopes_from_string",
        serialize_with = "scopes_to_string"
    )]
    pub scope: Vec<Scope>,
    pub valid_to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenListRequest {
    pub display_filter_option: DisplayFilterOption,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum DisplayFilterOption {
    All,
    #[default]
    Active,
    Expired,
    Revoked,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenGetRequest {
    pub authorization_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatTokenDeleteRequest {
    pub authorization_id: String,
}
