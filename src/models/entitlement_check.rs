// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    config_value::ConfigValue, entitlement_check_feature::EntitlementCheckFeature,
    entitlement_check_limit::EntitlementCheckLimit,
    entitlement_check_period::EntitlementCheckPeriod,
    entitlement_check_reason::EntitlementCheckReason,
};

/// Whether a customer may consume more of a feature now.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EntitlementCheck {
    /// When the usage behind the answer was measured.
    pub as_of: chrono::DateTime<chrono::Utc>,

    /// Always false: the answer reflects usage up to `as_of`.
    pub exact: bool,

    pub feature: EntitlementCheckFeature,

    pub has_access: bool,

    /// The entitlement allows overage (`ALLOW`) and the requested units go beyond its limit:
    /// access is granted, and the overage is billed.
    pub in_overage: bool,

    /// Age of that usage, in milliseconds.
    pub lag_ms: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<rust_decimal::Decimal>,

    /// Deprecated: always empty. Customer-specific limits are entitlement overrides, reported in
    /// `used`, `limit` and `remaining`.
    pub limits: Vec<EntitlementCheckLimit>,

    /// True when a hard limit is within 5% of being reached, reached, or refuses the request.
    /// While true, check before every request with its actual `requested`, and do not cache an
    /// answer that grants access.
    pub near_limit: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<EntitlementCheckPeriod>,

    pub reason: EntitlementCheckReason,

    /// Units left before the limit, grace excluded: 0 while `has_access` is true when usage is
    /// within the grace allowance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<rust_decimal::Decimal>,

    /// When to check again, in milliseconds. After a refusal by a limit whose period ends within
    /// 24 hours: the time until it ends (a smaller `requested` may still be allowed sooner);
    /// absent after a refusal by a sliding window, whose usage leaves it gradually. While
    /// `near_limit` is true and access is granted: 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<i64>,

    /// `used`, `limit`, `remaining` and `period` report the entitlement's limit. Absent when the
    /// entitlement has no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ConfigValue>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EntitlementCheck {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        as_of: chrono::DateTime<chrono::Utc>,
        exact: bool,
        feature: EntitlementCheckFeature,
        has_access: bool,
        in_overage: bool,
        lag_ms: i64,
        limits: Vec<EntitlementCheckLimit>,
        near_limit: bool,
        reason: EntitlementCheckReason,
    ) -> Self {
        Self {
            as_of,
            exact,
            feature,
            has_access,
            in_overage,
            lag_ms,
            limit: None,
            limits,
            near_limit,
            period: None,
            reason,
            remaining: None,
            retry_after_ms: None,
            used: None,
            value: None,
            extra: serde_json::Map::new(),
        }
    }
}
