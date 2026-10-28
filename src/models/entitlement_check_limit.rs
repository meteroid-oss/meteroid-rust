// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    entitlement_check_limit_behavior::EntitlementCheckLimitBehavior, usage_limit_id::UsageLimitId,
};

/// Deprecated: customer usage limits were replaced by entitlement overrides; never returned.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EntitlementCheckLimit {
    pub behavior: EntitlementCheckLimitBehavior,

    /// The group the limit was evaluated for, when the limit is grouped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::collections::HashMap<String, String>>,

    pub has_access: bool,

    pub limit: rust_decimal::Decimal,

    pub remaining: rust_decimal::Decimal,

    pub usage_limit_id: UsageLimitId,

    pub used: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl EntitlementCheckLimit {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        behavior: EntitlementCheckLimitBehavior,
        has_access: bool,
        limit: rust_decimal::Decimal,
        remaining: rust_decimal::Decimal,
        usage_limit_id: impl Into<UsageLimitId>,
        used: rust_decimal::Decimal,
    ) -> Self {
        Self {
            behavior,
            group: None,
            has_access,
            limit,
            remaining,
            usage_limit_id: usage_limit_id.into(),
            used,
            extra: serde_json::Map::new(),
        }
    }
}
