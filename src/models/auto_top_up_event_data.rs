// this file is @generated
use serde::{Deserialize, Serialize};

use super::customer_id::CustomerId;

/// An automatic top-up charge failed, or the automatic top-up was suspended.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AutoTopUpEventData {
    /// The charge in minor units of `currency`: the credit it would have added, or for a credit
    /// unit wallet the price of those credits in the customer's currency. When the credits could
    /// not be priced (no selling price), the credits themselves, with `currency` the unit code. On
    /// a suspension after a top-up payment was refunded or disputed: that payment's top-up.
    pub amount: i64,

    /// Failed charges in a row, this one included.
    pub consecutive_failures: i32,

    /// The currency charged (ISO 4217), or the credit unit code when the credits had no price.
    pub currency: String,

    pub customer_id: CustomerId,

    /// May carry payment provider detail.
    pub error: String,

    /// Set on `wallet.auto_top_up_suspended`: when it was suspended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AutoTopUpEventData {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        consecutive_failures: i32,
        currency: impl Into<String>,
        customer_id: impl Into<CustomerId>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            amount,
            consecutive_failures,
            currency: currency.into(),
            customer_id: customer_id.into(),
            error: error.into(),
            suspended_at: None,
            extra: serde_json::Map::new(),
        }
    }
}
