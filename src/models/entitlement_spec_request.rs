// this file is @generated
use serde::{Deserialize, Serialize};

use super::{entitlement_value::EntitlementValue, feature_id::FeatureId};

/// One entitlement to create: which feature, and the value granted by the entity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EntitlementSpecRequest {
    pub feature_id: FeatureId,

    pub value: EntitlementValue,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EntitlementSpecRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(feature_id: impl Into<FeatureId>, value: EntitlementValue) -> Self {
        Self {
            feature_id: feature_id.into(),
            value,
            extra: serde_json::Map::new(),
        }
    }
}
