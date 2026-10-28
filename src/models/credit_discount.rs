// this file is @generated
use serde::{Deserialize, Serialize};

use super::coupon_credit_scope::CouponCreditScope;

/// Promotional credit added to the customer's credit balance when the subscription the coupon is
/// redeemed on starts. It never discounts an invoice and applies before tax. A customer redeems
/// it at most once, so the coupon cannot be `reusable`. Its code is case-insensitive: it is stored
/// upper-case and redeemed whatever the case.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditDiscount {
    /// Credit added per redemption, in major units (e.g. "25.00").
    pub amount: String,

    /// ISO 4217 code; it must match the currency of the subscription the coupon is redeemed on.
    pub currency: String,

    /// Days the credit stays spendable after redemption, 1 to 3660. Absent: it never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<i32>,

    /// CUSTOMER (default): the credit pays any of the customer's subscriptions. SUBSCRIPTION: the
    /// credit pays only the subscription the code is redeemed on, and ends with it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CouponCreditScope>,

    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditDiscount {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: impl Into<String>, currency: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            expires_in_days: None,
            scope: None,
            r#type: "CREDIT".into(),
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `expires_in_days`.
    #[must_use]
    pub fn expires_in_days(mut self, expires_in_days: impl Into<i32>) -> Self {
        self.expires_in_days = Some(expires_in_days.into());
        self
    }

    /// Sets `scope`.
    #[must_use]
    pub fn scope(mut self, scope: impl Into<CouponCreditScope>) -> Self {
        self.scope = Some(scope.into());
        self
    }
}
