// this file is @generated
use serde::{Deserialize, Serialize};

/// Partial update. Code, feature type and product are immutable.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFeatureRequest {
    /// Omit to leave unchanged; send `null` to clear.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub description: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub name: Option<Option<String>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UpdateFeatureRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            name: None,
            extra: serde_json::Map::new(),
        }
    }
}
