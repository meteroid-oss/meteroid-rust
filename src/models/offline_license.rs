// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct OfflineLicense {
    /// The signed license (a JWT) to install in the air-gapped installation.
    pub license: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl OfflineLicense {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(license: impl Into<String>) -> Self {
        Self {
            license: license.into(),
            extra: serde_json::Map::new(),
        }
    }
}
