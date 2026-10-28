// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billable_metric_id::BillableMetricId, fee_credit_unit::FeeCreditUnit};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CapacityFee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<FeeCreditUnit>,

    pub included: i64,

    pub metric_id: BillableMetricId,

    pub overage_rate: rust_decimal::Decimal,

    pub rate: rust_decimal::Decimal,

    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CapacityFee {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        included: i64,
        metric_id: impl Into<BillableMetricId>,
        overage_rate: rust_decimal::Decimal,
        rate: rust_decimal::Decimal,
    ) -> Self {
        Self {
            credit_unit: None,
            included,
            metric_id: metric_id.into(),
            overage_rate,
            rate,
            r#type: "CAPACITY".into(),
            extra: serde_json::Map::new(),
        }
    }
}
