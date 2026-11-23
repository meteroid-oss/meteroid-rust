// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EventChangeResponse {
    /// RFC 3339 stamp of the queued change, with nanoseconds. Changes to the same event resolve by
    /// this stamp: the latest wins.
    pub ingested_at: chrono::DateTime<chrono::Utc>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EventChangeResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(ingested_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            ingested_at,
            extra: serde_json::Map::new(),
        }
    }
}
