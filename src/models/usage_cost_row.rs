// this file is @generated
use serde::{Deserialize, Serialize};

use super::usage_cost::UsageCost;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageCostRow {
    pub bucket_start: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<UsageCost>,

    /// False when the price depends on the whole period's usage (tiered, volume, package,
    /// capacity): the cost is then what the bucket adds to the period's price so far. A period's
    /// rows still add up to its invoiced amount.
    pub exact: bool,

    /// The value of `group_by` for the row. Absent when the request is not grouped, or the usage
    /// has no value for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    pub quantity: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageCostRow {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        bucket_start: chrono::DateTime<chrono::Utc>,
        exact: bool,
        quantity: rust_decimal::Decimal,
    ) -> Self {
        Self {
            bucket_start,
            cost: None,
            exact,
            group: None,
            quantity,
            extra: serde_json::Map::new(),
        }
    }
}
