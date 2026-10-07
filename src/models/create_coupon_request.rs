// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_discount::CouponDiscount, plan_id::PlanId};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateCouponRequest {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub discount: CouponDiscount,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ids: Option<Vec<PlanId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_value: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateCouponRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(code: impl Into<String>, discount: CouponDiscount) -> Self {
        Self {
            code: code.into(),
            description: None,
            discount,
            expires_at: None,
            plan_ids: None,
            recurring_value: None,
            redemption_limit: None,
            reusable: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, expires_at: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    /// Sets `plan_ids`.
    #[must_use]
    pub fn plan_ids(mut self, plan_ids: impl Into<Vec<PlanId>>) -> Self {
        self.plan_ids = Some(plan_ids.into());
        self
    }

    /// Sets `recurring_value`.
    #[must_use]
    pub fn recurring_value(mut self, recurring_value: impl Into<i32>) -> Self {
        self.recurring_value = Some(recurring_value.into());
        self
    }

    /// Sets `redemption_limit`.
    #[must_use]
    pub fn redemption_limit(mut self, redemption_limit: impl Into<i32>) -> Self {
        self.redemption_limit = Some(redemption_limit.into());
        self
    }

    /// Sets `reusable`.
    #[must_use]
    pub fn reusable(mut self, reusable: impl Into<bool>) -> Self {
        self.reusable = Some(reusable.into());
        self
    }
}
