// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    entitlement_value::EntitlementValue, feature_type::FeatureType, product_id::ProductId,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateFeatureRequest {
    /// Unique key used to reference this feature in your code. Cannot be changed after creation.
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<EntitlementValue>,

    /// Fixed at creation — a feature never changes type.
    pub feature_type: FeatureType,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateFeatureRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        feature_type: FeatureType,
        name: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            description: None,
            entitlement: None,
            feature_type,
            name: name.into(),
            product_id: None,
            extra: serde_json::Map::new(),
        }
    }
}
