// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, grouped_usage::GroupedUsage, usage_cost::UsageCost,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MetricUsage {
    /// Beta: what the metric's usage adds so far to the subscription's next invoice, per currency
    /// or credit unit; empty while it bills nothing, as during a free trial or once cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<Vec<UsageCost>>,

    pub grouped_usage: Vec<GroupedUsage>,

    pub metric_code: String,

    pub metric_id: BillableMetricId,

    pub metric_name: String,

    pub total_value: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl MetricUsage {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        grouped_usage: Vec<GroupedUsage>,
        metric_code: impl Into<String>,
        metric_id: impl Into<BillableMetricId>,
        metric_name: impl Into<String>,
        total_value: rust_decimal::Decimal,
    ) -> Self {
        Self {
            costs: None,
            grouped_usage,
            metric_code: metric_code.into(),
            metric_id: metric_id.into(),
            metric_name: metric_name.into(),
            total_value,
            extra: serde_json::Map::new(),
        }
    }
}
