// this file is @generated
use serde::{Deserialize, Serialize};

use super::{license_signing_key::LicenseSigningKey, licensing_endpoints::LicensingEndpoints};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LicenseSigningKeysResponse {
    pub endpoints: LicensingEndpoints,

    /// Active key first. The first key is created on first issue.
    pub keys: Vec<LicenseSigningKey>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LicenseSigningKeysResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(endpoints: LicensingEndpoints, keys: Vec<LicenseSigningKey>) -> Self {
        Self {
            endpoints,
            keys,
            extra: serde_json::Map::new(),
        }
    }
}
