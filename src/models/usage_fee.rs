// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, fee_credit_unit::FeeCreditUnit,
    usage_pricing_model::UsagePricingModel,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageFee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<FeeCreditUnit>,

    pub metric_id: BillableMetricId,

    pub model: UsagePricingModel,

    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageFee {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(metric_id: impl Into<BillableMetricId>, model: UsagePricingModel) -> Self {
        Self {
            credit_unit: None,
            metric_id: metric_id.into(),
            model,
            r#type: "USAGE".into(),
            extra: serde_json::Map::new(),
        }
    }
}
