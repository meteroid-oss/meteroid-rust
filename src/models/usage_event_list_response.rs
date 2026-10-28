// this file is @generated
use serde::{Deserialize, Serialize};

use super::usage_event::UsageEvent;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageEventListResponse {
    pub data: Vec<UsageEvent>,

    /// Absent on the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageEventListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<UsageEvent>) -> Self {
        Self {
            data,
            next_cursor: None,
            extra: serde_json::Map::new(),
        }
    }
}
