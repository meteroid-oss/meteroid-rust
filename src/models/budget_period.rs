// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BudgetPeriod {
    /// Absent when the period never ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::NaiveDate>,

    pub start: chrono::NaiveDate,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl BudgetPeriod {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(start: chrono::NaiveDate) -> Self {
        Self {
            end: None,
            start,
            extra: serde_json::Map::new(),
        }
    }
}
