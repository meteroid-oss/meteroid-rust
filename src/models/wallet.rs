// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    auto_top_up::AutoTopUp, currency::Currency, subscription_credit::SubscriptionCredit,
    usage_at_zero::UsageAtZero,
};

/// A customer's credit in one currency or credit unit. Amounts are minor units of it. Read with
/// `subscription_id`, every amount counts only the credit that subscription can use.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Wallet {
    /// When `balance` and `unbilled_usage` were computed. They follow usage within a few seconds
    /// while the balance is near zero or a threshold, and within a minute otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_top_up: Option<AutoTopUp>,

    /// What the customer has left: `credit_on_account` minus the part of `unbilled_usage` that
    /// credit will pay. Never negative: usage beyond the credit stays in `unbilled_usage`, to be
    /// invoiced. Low-balance alerts use it; automatic top-ups watch only its part that pays any
    /// charge (credit that belongs to no subscription and is not limited to usage or products).
    /// Absent, with `unbilled_usage` and `as_of`, until the customer's usage has been priced once.
    /// With `subscription_id`: the credit that subscription can spend once every subscription's
    /// unbilled usage is paid, minus its own unbilled usage no credit pays (negative then), as
    /// `GET /wallets/check` reads it. Credit restricted to products it is not charged for is
    /// left out. Credit that belongs to no subscription counts in the balance of every
    /// subscription that can use it (see `shared_balance`): never add subscriptions' balances up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// The ledger total: credit granted and effective, not yet applied to an invoice or expired,
    /// whatever it applies to. Usage not invoiced yet is not deducted (see `balance`).
    pub credit_on_account: i64,

    /// The part of `credit_on_account` that pays only usage charges.
    pub credit_on_account_usage_only: i64,

    /// The credit unit (its code) the wallet holds; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: amounts are integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Ascending, distinct, at most 5. `wallet.balance_low` fires once when `balance` falls below
    /// one; a drop below several at once fires once, for the lowest. All credit counts, including
    /// credit that belongs to a subscription or is limited to usage or products, which
    /// `wallet.depleted` leaves out: a wallet can be depleted without an alert firing.
    pub low_balance_thresholds: Vec<i64>,

    /// Usage stops as soon as the credit is used up, for at least one active subscription: by
    /// `usage_at_zero`, or by the plan of a subscription charging usage in this wallet.
    pub pauses_at_zero: bool,

    /// With `subscription_id` only: the part of `balance` that belongs to no subscription, which
    /// the customer's other subscriptions can spend too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_balance: Option<i64>,

    /// The part of `credit_on_account` that belongs to a subscription, which only that
    /// subscription's charges can use, per subscription. The rest pays any subscription.
    pub subscription_credit: Vec<SubscriptionCredit>,

    /// Usage recorded and priced but not invoiced yet, tax included, in minor units: what the
    /// next invoices will charge for it, paid from credit as far as the credit goes. With
    /// `subscription_id`, that subscription's usage only; other subscriptions' usage paid from
    /// shared credit still lowers `balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbilled_usage: Option<i64>,

    /// Credit unit wallets: the price this customer agreed for one credit, in major units of
    /// `unit_price_currency`, as a decimal string (e.g. "0.0008"). Purchases and automatic
    /// top-ups in that currency charge it instead of the list price, unless a purchase sets its
    /// own `unit_price`. Absent: the list price applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_currency: Option<Currency>,

    /// Credit granted that takes effect later.
    pub upcoming_credit: i64,

    /// What this customer's usage does once the credit a subscription can use for it is used up
    /// (its own credit, credit that belongs to no subscription, and credit limited to the product:
    /// the `balance` of `GET /wallets/check`). `FOLLOW_PLAN` applies the "when credit runs out"
    /// rule of each subscription's plan.
    pub usage_at_zero: UsageAtZero,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Wallet {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        credit_on_account: i64,
        credit_on_account_usage_only: i64,
        low_balance_thresholds: Vec<i64>,
        pauses_at_zero: bool,
        subscription_credit: Vec<SubscriptionCredit>,
        upcoming_credit: i64,
        usage_at_zero: UsageAtZero,
    ) -> Self {
        Self {
            as_of: None,
            auto_top_up: None,
            balance: None,
            credit_on_account,
            credit_on_account_usage_only,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            low_balance_thresholds,
            pauses_at_zero,
            shared_balance: None,
            subscription_credit,
            unbilled_usage: None,
            unit_price: None,
            unit_price_currency: None,
            upcoming_credit,
            usage_at_zero,
            extra: serde_json::Map::new(),
        }
    }
}
