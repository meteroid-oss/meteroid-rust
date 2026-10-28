// this file is @generated
use serde::{Deserialize, Serialize};

/// Never resets — counts all usage since the subscription was activated.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NeverResetPeriod {
    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl NeverResetPeriod {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r#type: "NEVER".into(),
            extra: serde_json::Map::new(),
        }
    }
}

/// Sets `type` to `NEVER`.
impl Default for NeverResetPeriod {
    fn default() -> Self {
        Self {
            r#type: "NEVER".into(),
            extra: serde_json::Map::new(),
        }
    }
}
