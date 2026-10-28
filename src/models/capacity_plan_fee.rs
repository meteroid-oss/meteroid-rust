// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, billing_period_enum::BillingPeriodEnum,
    capacity_threshold::CapacityThreshold, plan_fee_credit_unit::PlanFeeCreditUnit,
};

/// Capacity-based fee with included committed usage and overage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CapacityPlanFee {
    pub cadence: BillingPeriodEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<PlanFeeCreditUnit>,

    pub metric_id: BillableMetricId,

    pub thresholds: Vec<CapacityThreshold>,

    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CapacityPlanFee {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        cadence: BillingPeriodEnum,
        metric_id: impl Into<BillableMetricId>,
        thresholds: Vec<CapacityThreshold>,
    ) -> Self {
        Self {
            cadence,
            credit_unit: None,
            metric_id: metric_id.into(),
            thresholds,
            r#type: "CAPACITY".into(),
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `credit_unit`.
    #[must_use]
    pub fn credit_unit(mut self, credit_unit: impl Into<PlanFeeCreditUnit>) -> Self {
        self.credit_unit = Some(credit_unit.into());
        self
    }
}
