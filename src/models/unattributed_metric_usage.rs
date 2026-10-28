// this file is @generated
use serde::{Deserialize, Serialize};

use super::billable_metric_id::BillableMetricId;

/// A metric's usage over `[from, until)` that no subscription bills: it matches none of the
/// usage filters of the subscriptions pricing the metric, and none of them bills without one.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UnattributedMetricUsage {
    pub from: chrono::DateTime<chrono::Utc>,

    pub metric_code: String,

    pub metric_id: BillableMetricId,

    pub metric_name: String,

    pub total_value: rust_decimal::Decimal,

    pub until: chrono::DateTime<chrono::Utc>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UnattributedMetricUsage {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        from: chrono::DateTime<chrono::Utc>,
        metric_code: impl Into<String>,
        metric_id: impl Into<BillableMetricId>,
        metric_name: impl Into<String>,
        total_value: rust_decimal::Decimal,
        until: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            from,
            metric_code: metric_code.into(),
            metric_id: metric_id.into(),
            metric_name: metric_name.into(),
            total_value,
            until,
            extra: serde_json::Map::new(),
        }
    }
}
