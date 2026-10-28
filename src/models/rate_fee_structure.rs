// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RateFeeStructure {
    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl RateFeeStructure {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r#type: "RATE".into(),
            extra: serde_json::Map::new(),
        }
    }
}

/// Sets `type` to `RATE`.
impl Default for RateFeeStructure {
    fn default() -> Self {
        Self {
            r#type: "RATE".into(),
            extra: serde_json::Map::new(),
        }
    }
}
