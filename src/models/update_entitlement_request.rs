// this file is @generated
use serde::{Deserialize, Serialize};

use super::entitlement_value::EntitlementValue;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateEntitlementRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub value: Option<Option<EntitlementValue>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UpdateEntitlementRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: None,
            extra: serde_json::Map::new(),
        }
    }
}
