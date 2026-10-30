// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LicenseActivationCode {
    pub activation_code: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LicenseActivationCode {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(activation_code: impl Into<String>) -> Self {
        Self {
            activation_code: activation_code.into(),
            extra: serde_json::Map::new(),
        }
    }
}
