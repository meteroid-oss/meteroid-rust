// this file is @generated
use serde::{Deserialize, Serialize};

use super::usage_cost_row::UsageCostRow;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageCostListResponse {
    pub data: Vec<UsageCostRow>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageCostListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<UsageCostRow>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
