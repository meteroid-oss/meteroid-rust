// this file is @generated
use serde::{Deserialize, Serialize};

use super::{metric_usage::MetricUsage, usage_cost::UsageCost};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageResponse {
    /// Beta: when `totals` and the metrics' `costs` were computed. These are present only when a
    /// subscription's current billing period is read (no `start_date` nor `end_date`) and its
    /// usage could be priced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_at: Option<chrono::DateTime<chrono::Utc>>,

    pub period_end: chrono::NaiveDate,

    pub period_start: chrono::NaiveDate,

    /// Beta: the sum of the metrics' `costs`, per currency or credit unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<UsageCost>>,

    pub usage: Vec<MetricUsage>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        period_end: chrono::NaiveDate,
        period_start: chrono::NaiveDate,
        usage: Vec<MetricUsage>,
    ) -> Self {
        Self {
            computed_at: None,
            period_end,
            period_start,
            totals: None,
            usage,
            extra: serde_json::Map::new(),
        }
    }
}
