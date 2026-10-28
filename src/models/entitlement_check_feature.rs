// this file is @generated
use serde::{Deserialize, Serialize};

use super::feature_id::FeatureId;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EntitlementCheckFeature {
    pub code: String,

    pub id: FeatureId,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EntitlementCheckFeature {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(code: impl Into<String>, id: impl Into<FeatureId>) -> Self {
        Self {
            code: code.into(),
            id: id.into(),
            extra: serde_json::Map::new(),
        }
    }
}
