// this file is @generated
use serde::{Deserialize, Serialize};

use super::license::License;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct IssueLicenseResponse {
    /// Online: one-time code the customer enters in their installation to activate the license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_code: Option<String>,

    pub license: License,

    /// Offline: the signed license (a JWT) to install in the installation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_license: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl IssueLicenseResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(license: License) -> Self {
        Self {
            activation_code: None,
            license,
            offline_license: None,
            extra: serde_json::Map::new(),
        }
    }
}
