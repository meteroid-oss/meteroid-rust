// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, entitlement_overage_behavior::EntitlementOverageBehavior,
    reset_period::ResetPeriod,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MeteredEntitlementSpec {
    /// False when switched off, or once a `BLOCK` limit (plus its grace) is reached. An `ALLOW`
    /// limit never turns it off.
    pub enabled: bool,

    /// `BLOCK` only: usage allowed beyond the limit before it is refused, in percent of it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_pct: Option<i32>,

    /// Event property the limit applies to separately; usage is then the largest value's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<rust_decimal::Decimal>,

    pub metric_id: BillableMetricId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_behavior: Option<EntitlementOverageBehavior>,

    pub reset_period: ResetPeriod,

    /// Percents of the limit at which an alert is sent, ascending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<Vec<i32>>,

    /// Deprecated: the lowest of `thresholds`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_threshold_pct: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl MeteredEntitlementSpec {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        enabled: bool,
        metric_id: impl Into<BillableMetricId>,
        reset_period: ResetPeriod,
    ) -> Self {
        Self {
            enabled,
            grace_pct: None,
            group_by: None,
            limit: None,
            metric_id: metric_id.into(),
            overage_behavior: None,
            reset_period,
            thresholds: None,
            warning_threshold_pct: None,
            extra: serde_json::Map::new(),
        }
    }
}
