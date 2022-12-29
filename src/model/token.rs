use chrono::{DateTime, Utc};
use pattrick_clap::Scope;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::model::scope::ScopeDef;

#[derive(Tabled, Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PatToken {
    #[serde(rename = "authorizationId")]
    pub id: String,
    pub display_name: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    #[serde(with = "ScopeDef")]
    pub scope: Scope,
    #[tabled(display_with = "display_token")]
    pub token: Option<String>,
}

fn display_token(token: &Option<String>) -> String {
    match token {
        Some(token) => token.to_string(),
        None => "N/A".to_string(),
    }
}
