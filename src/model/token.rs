use chrono::{DateTime, Utc};
use pattrick_clap::Scope;
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
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
    #[tabled(display_with = "display_scopes")]
    #[serde(
        deserialize_with = "scopes_from_string",
        serialize_with = "scopes_to_string"
    )]
    pub scope: Vec<Scope>,
    #[tabled(display_with = "display_token")]
    pub token: Option<String>,
}

fn display_token(token: &Option<String>) -> String {
    match token {
        Some(token) => token.to_string(),
        None => "N/A".to_string(),
    }
}

fn display_scopes(scopes: &Vec<Scope>) -> String {
    let mut scope_string = String::new();
    for scope in scopes {
        scope_string.push_str(&format!("{scope} "));
    }
    scope_string
}

fn scopes_to_string<S>(scopes: &Vec<Scope>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&display_scopes(scopes))
}

fn scopes_from_string<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let scopes = s
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| {
            let scope: Result<Scope, serde::de::value::Error> =
                ScopeDef::deserialize(s.into_deserializer());
            scope.unwrap_or_else(|_| panic!("Failed to deserialize scope: {s}"))
        })
        .collect();
    Ok(scopes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_scopes_single() {
        let test_case: &str = "vso.packaging";
        let deserializer: serde::de::value::StrDeserializer<'_, serde::de::value::Error> =
            test_case.into_deserializer();
        assert_eq!(
            scopes_from_string(deserializer).unwrap(),
            vec![Scope::Packaging]
        )
    }

    #[test]
    fn test_deserialize_scopes_multiple() {
        let test_case: &str = "vso.packaging vso.code_write";
        let deserializer: serde::de::value::StrDeserializer<'_, serde::de::value::Error> =
            test_case.into_deserializer();
        assert_eq!(
            scopes_from_string(deserializer).unwrap(),
            vec![Scope::Packaging, Scope::CodeWrite]
        )
    }
}
