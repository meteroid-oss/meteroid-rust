// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EntitlementCheckPeriod {
    /// Absent when the period never ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,

    pub start: chrono::DateTime<chrono::Utc>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EntitlementCheckPeriod {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(start: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            end: None,
            start,
            extra: serde_json::Map::new(),
        }
    }
}
