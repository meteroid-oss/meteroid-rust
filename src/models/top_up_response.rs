// this file is @generated
use serde::{Deserialize, Serialize};

use super::{currency::Currency, top_up_status::TopUpStatus};

/// Where a top-up stands. Its credit is granted once it is paid; `wallet.funded` reports it.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TopUpResponse {
    /// Credit bought, in minor units of `currency`, or of `credit_unit` when set.
    pub amount: i64,

    /// Charged to the payment method, tax included, in minor units of `currency`; absent until a
    /// charge is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charged_amount: Option<i64>,

    /// Hosted page where the customer completes the payment; present while it awaits them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,

    /// Set when credits of this credit unit were bought: `amount` is in its minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// The money the top-up is charged in.
    pub currency: Currency,

    /// The invoice documenting the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    /// The charge made on the payment method; absent until a charge is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_transaction_id: Option<String>,

    /// SUCCEEDED once paid; otherwise what the payment is waiting for.
    pub status: TopUpStatus,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl TopUpResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64, currency: Currency, status: TopUpStatus) -> Self {
        Self {
            amount,
            charged_amount: None,
            checkout_url: None,
            credit_unit: None,
            currency,
            invoice_id: None,
            payment_transaction_id: None,
            status,
            extra: serde_json::Map::new(),
        }
    }
}
