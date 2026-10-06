// this file is @generated
use serde::{Deserialize, Serialize};

use super::entitlement_spec_request::EntitlementSpecRequest;

/// Entitlements already present on the entity are skipped, so the call can be replayed.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateEntitlementsRequest {
    pub entitlements: Vec<EntitlementSpecRequest>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateEntitlementsRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(entitlements: Vec<EntitlementSpecRequest>) -> Self {
        Self {
            entitlements,
            extra: serde_json::Map::new(),
        }
    }
}
