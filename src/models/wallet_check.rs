// this file is @generated
use serde::{Deserialize, Serialize};

use super::currency::Currency;

/// Whether a customer may keep consuming what their prepaid credit pays for.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletCheck {
    /// When `balance` was computed.
    pub as_of: chrono::DateTime<chrono::Utc>,

    /// The balance: credit on account net of the usage not invoiced yet that it will pay, in
    /// minor units of `currency` or `credit_unit`; never negative for the customer as a whole,
    /// as `balance` of `GET /wallets`. With `product_id`, only the credit that can pay for that
    /// product, net of its usage no credit covers. With `subscription_id`, only the credit that
    /// subscription can spend (credit restricted to products it is not charged for left out),
    /// net of its usage no credit covers; credit that belongs to no subscription counts in every
    /// subscription's balance (see `shared_balance`).
    pub balance: i64,

    /// The credit unit (its code) checked; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: `balance` is in integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Always false: `balance` may lag usage by a few seconds (see `as_of`), and the answer is
    /// cached for one second.
    pub exact: bool,

    /// True once `balance` is at or below zero, or, with `subscription_id`, once no credit is
    /// left for some of its usage. Independent of `has_access`: a wallet that allows overage
    /// still reports this, so the caller can warn without refusing.
    pub exhausted: bool,

    /// False only once `balance` is at or below zero and `blocks_when_exhausted` is true.
    /// Customers without a wallet in `currency` do not prepay, and always have access. With
    /// `subscription_id`: false when `blocks_when_exhausted` is true and no credit that
    /// subscription can spend is left for some of its usage (of `product_id` when given), as the
    /// access gate refuses it; a subscription charged no usage in the wallet keeps access.
    pub has_access: bool,

    /// Age of `balance`, in milliseconds.
    pub lag_ms: i64,

    /// Whether usage is refused as soon as the credit is used up: the wallet's `usage_at_zero`,
    /// else the plan of the subscription checked (of any active subscription without one). A plan
    /// that invoices usage up to a cap refuses it past the cap only, as `has_access` reports.
    pub pauses_at_zero: bool,

    /// With `subscription_id` only: the part of `balance` that belongs to no subscription, which
    /// the customer's other subscriptions can spend too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_balance: Option<i64>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletCheck {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        as_of: chrono::DateTime<chrono::Utc>,
        balance: i64,
        exact: bool,
        exhausted: bool,
        has_access: bool,
        lag_ms: i64,
        pauses_at_zero: bool,
    ) -> Self {
        Self {
            as_of,
            balance,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            exact,
            exhausted,
            has_access,
            lag_ms,
            pauses_at_zero,
            shared_balance: None,
            extra: serde_json::Map::new(),
        }
    }
}
