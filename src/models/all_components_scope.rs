// this file is @generated
use serde::{Deserialize, Serialize};

/// Every revenue line counts toward the floor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AllComponentsScope {
    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AllComponentsScope {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r#type: "all_components".into(),
            extra: serde_json::Map::new(),
        }
    }
}

/// Sets `type` to `all_components`.
impl Default for AllComponentsScope {
    fn default() -> Self {
        Self {
            r#type: "all_components".into(),
            extra: serde_json::Map::new(),
        }
    }
}
