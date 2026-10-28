// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OneTimeFeeStructure {
    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl OneTimeFeeStructure {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r#type: "ONE_TIME".into(),
            extra: serde_json::Map::new(),
        }
    }
}

/// Sets `type` to `ONE_TIME`.
impl Default for OneTimeFeeStructure {
    fn default() -> Self {
        Self {
            r#type: "ONE_TIME".into(),
            extra: serde_json::Map::new(),
        }
    }
}
