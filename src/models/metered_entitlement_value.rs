// this file is @generated
use serde::{Deserialize, Serialize};

use super::{entitlement_overage_behavior::EntitlementOverageBehavior, reset_period::ResetPeriod};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MeteredEntitlementValue {
    /// Per-entitlement kill switch. `false` means disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// `BLOCK` only: usage allowed beyond the limit before it is refused, in percent of the
    /// limit (1 to 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_pct: Option<i32>,

    /// Event property the limit applies to separately, one limit per value (e.g. `user_id`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,

    /// Cap on usage. Null means unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_behavior: Option<EntitlementOverageBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_period: Option<ResetPeriod>,

    /// Percents of the limit (1 to 100, at most 10) at which an alert is sent, once per period:
    /// `entitlement.usage_threshold_reached`, or `entitlement.limit_reached` at 100. Requires a
    /// limit and a window that is not sliding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<Vec<i32>>,

    pub r#type: String,

    /// Deprecated: use `thresholds`. A single alert threshold, read only when `thresholds` is
    /// omitted; returned as the lowest of `thresholds`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_threshold_pct: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl MeteredEntitlementValue {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            enabled: None,
            grace_pct: None,
            group_by: None,
            limit: None,
            overage_behavior: None,
            reset_period: None,
            thresholds: None,
            r#type: "METERED".into(),
            warning_threshold_pct: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }

    /// Sets `grace_pct`.
    #[must_use]
    pub fn grace_pct(mut self, grace_pct: impl Into<i32>) -> Self {
        self.grace_pct = Some(grace_pct.into());
        self
    }

    /// Sets `group_by`.
    #[must_use]
    pub fn group_by(mut self, group_by: impl Into<String>) -> Self {
        self.group_by = Some(group_by.into());
        self
    }

    /// Sets `limit`.
    #[must_use]
    pub fn limit(mut self, limit: impl Into<rust_decimal::Decimal>) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Sets `overage_behavior`.
    #[must_use]
    pub fn overage_behavior(
        mut self,
        overage_behavior: impl Into<EntitlementOverageBehavior>,
    ) -> Self {
        self.overage_behavior = Some(overage_behavior.into());
        self
    }

    /// Sets `reset_period`.
    #[must_use]
    pub fn reset_period(mut self, reset_period: impl Into<ResetPeriod>) -> Self {
        self.reset_period = Some(reset_period.into());
        self
    }

    /// Sets `thresholds`.
    #[must_use]
    pub fn thresholds(mut self, thresholds: impl Into<Vec<i32>>) -> Self {
        self.thresholds = Some(thresholds.into());
        self
    }

    /// Sets `warning_threshold_pct`.
    #[must_use]
    pub fn warning_threshold_pct(mut self, warning_threshold_pct: impl Into<i32>) -> Self {
        self.warning_threshold_pct = Some(warning_threshold_pct.into());
        self
    }
}

/// Sets `type` to `METERED`.
impl Default for MeteredEntitlementValue {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            grace_pct: Default::default(),
            group_by: Default::default(),
            limit: Default::default(),
            overage_behavior: Default::default(),
            reset_period: Default::default(),
            thresholds: Default::default(),
            r#type: "METERED".into(),
            warning_threshold_pct: Default::default(),
            extra: serde_json::Map::new(),
        }
    }
}
