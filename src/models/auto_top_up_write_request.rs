// this file is @generated
use serde::{Deserialize, Serialize};

use super::auto_top_up_method::AutoTopUpMethod;

/// An automatic top-up's terms, replaced as a whole. FIXED needs `amount` and no `target`;
/// TO_TARGET needs `target` above `threshold` and no `amount`; amounts are positive.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AutoTopUpWriteRequest {
    /// FIXED only: credit bought each time, in minor units of the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Credit unit wallets: authorize today's price per credit for these terms, which lifts a
    /// stop caused by a price increase. When false (the default) the terms keep the price they
    /// were authorized at, and a rule stopped by a price increase stays stopped.
    /// A price at or below the authorized one never stops the rule: top-ups charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_current_price: Option<bool>,

    /// Attests that the customer agreed to automatic charges on these terms; recorded with the
    /// API key and the time. Required (true) when the terms turn automatic top-up on, including
    /// one the customer turned off, or change what it charges. Not needed to resend the stored
    /// terms unchanged, nor to change terms the customer confirmed in their portal: those stop
    /// until the customer confirms the new ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_consent_attested: Option<bool>,

    /// Most charged in a rolling day, tax included. For a credit unit wallet caps are money, in
    /// minor units of the customer's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<i64>,

    /// Largest single charge, in the same money as `minimum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    /// FIXED buys `amount` each time; TO_TARGET buys back up to `target`.
    pub method: AutoTopUpMethod,

    /// Smallest single charge. For a credit unit wallet it is money (the price before tax, in
    /// minor units of the customer's currency), like the caps; the threshold, amount and target
    /// are credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,

    /// Most charged in a calendar month (UTC), tax included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_cap: Option<i64>,

    /// Defaults to the customer's default payment method: the customer's agreement, and your
    /// attestation of it, then cover whichever method is their default at each top-up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,

    /// TO_TARGET only: the balance a top-up brings the wallet back to; must exceed `threshold`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<i64>,

    /// Tops up once the balance is at or below this, in minor units of the wallet.
    pub threshold: i64,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AutoTopUpWriteRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(method: AutoTopUpMethod, threshold: i64) -> Self {
        Self {
            amount: None,
            at_current_price: None,
            customer_consent_attested: None,
            daily_cap: None,
            maximum: None,
            method,
            minimum: None,
            monthly_cap: None,
            payment_method_id: None,
            target: None,
            threshold,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `amount`.
    #[must_use]
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Sets `at_current_price`.
    #[must_use]
    pub fn at_current_price(mut self, at_current_price: impl Into<bool>) -> Self {
        self.at_current_price = Some(at_current_price.into());
        self
    }

    /// Sets `customer_consent_attested`.
    #[must_use]
    pub fn customer_consent_attested(mut self, customer_consent_attested: impl Into<bool>) -> Self {
        self.customer_consent_attested = Some(customer_consent_attested.into());
        self
    }

    /// Sets `daily_cap`.
    #[must_use]
    pub fn daily_cap(mut self, daily_cap: impl Into<i64>) -> Self {
        self.daily_cap = Some(daily_cap.into());
        self
    }

    /// Sets `maximum`.
    #[must_use]
    pub fn maximum(mut self, maximum: impl Into<i64>) -> Self {
        self.maximum = Some(maximum.into());
        self
    }

    /// Sets `minimum`.
    #[must_use]
    pub fn minimum(mut self, minimum: impl Into<i64>) -> Self {
        self.minimum = Some(minimum.into());
        self
    }

    /// Sets `monthly_cap`.
    #[must_use]
    pub fn monthly_cap(mut self, monthly_cap: impl Into<i64>) -> Self {
        self.monthly_cap = Some(monthly_cap.into());
        self
    }

    /// Sets `payment_method_id`.
    #[must_use]
    pub fn payment_method_id(mut self, payment_method_id: impl Into<String>) -> Self {
        self.payment_method_id = Some(payment_method_id.into());
        self
    }

    /// Sets `target`.
    #[must_use]
    pub fn target(mut self, target: impl Into<i64>) -> Self {
        self.target = Some(target.into());
        self
    }
}
