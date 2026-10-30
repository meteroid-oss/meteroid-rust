// this file is @generated
use serde::{Deserialize, Serialize};

use super::license::License;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LicenseListResponse {
    pub data: Vec<License>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LicenseListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<License>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
