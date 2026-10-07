// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_discount::CouponDiscount, plan_id::PlanId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateCouponRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub description: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub discount: Option<Option<CouponDiscount>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub plan_ids: Option<Option<Vec<PlanId>>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UpdateCouponRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            discount: None,
            plan_ids: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(Some(description.into()));
        self
    }

    /// Sends `description` as `null`, clearing it.
    #[must_use]
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// Sets `discount`.
    #[must_use]
    pub fn discount(mut self, discount: impl Into<CouponDiscount>) -> Self {
        self.discount = Some(Some(discount.into()));
        self
    }

    /// Sends `discount` as `null`, clearing it.
    #[must_use]
    pub fn clear_discount(mut self) -> Self {
        self.discount = Some(None);
        self
    }

    /// Sets `plan_ids`.
    #[must_use]
    pub fn plan_ids(mut self, plan_ids: impl Into<Vec<PlanId>>) -> Self {
        self.plan_ids = Some(Some(plan_ids.into()));
        self
    }

    /// Sends `plan_ids` as `null`, clearing it.
    #[must_use]
    pub fn clear_plan_ids(mut self) -> Self {
        self.plan_ids = Some(None);
        self
    }
}
