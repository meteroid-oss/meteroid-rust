// this file is @generated
use serde::{Deserialize, Serialize};

use super::usage_cost::UsageCost;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<UsageCost>,

    pub id: String,

    pub properties: std::collections::HashMap<String, String>,

    /// Billed units the event adds. Absent when the metric's aggregation cannot be split per
    /// event, or when the event does not count towards the metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<rust_decimal::Decimal>,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        properties: std::collections::HashMap<String, String>,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            cost: None,
            id: id.into(),
            properties,
            quantity: None,
            timestamp,
            extra: serde_json::Map::new(),
        }
    }
}
