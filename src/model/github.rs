use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GitHubRelease {
    url: String,
    assets_url: String,
    pub tag_name: String,
    published_at: DateTime<Utc>,
}
