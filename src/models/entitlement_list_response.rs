// this file is @generated
use serde::{Deserialize, Serialize};

use super::entitlement::Entitlement;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EntitlementListResponse {
    pub data: Vec<Entitlement>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EntitlementListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<Entitlement>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
