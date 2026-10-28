// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_applies_to::CreditAppliesTo, credit_cost_request::CreditCostRequest,
    credit_grant_type::CreditGrantType, currency::Currency, product_id::ProductId,
};

/// Credit granted without charging or invoicing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateCreditGrantRequest {
    /// Credit granted, in minor units of `currency`, or of `credit_unit` when set (e.g. 500000
    /// is 5,000.00 credits of a unit with precision 2). Positive.
    pub amount: i64,

    /// Restricts the credit to charges of these products; absent applies to every product.
    /// Combined with `applies_to`. Not available for credit units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_product_ids: Option<Vec<ProductId>>,

    /// ALL pays any charge on an invoice; USAGE only usage charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<CreditAppliesTo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<CreditCostRequest>,

    /// Grants in this credit unit (its code) instead of a currency; excludes `currency`. Credit in
    /// a credit unit pays the usage priced in that unit, so `applies_to` must stay ALL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// When unspent credit lapses; absent never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Lower values are drawn first among grants of the same type that pay the same charges;
    /// defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Shown on the credit ledger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Your own reference, e.g. a contract or support ticket. Required for PAID credit that cost
    /// more than zero when the customer's invoicing entity is in France: the number of the invoice
    /// or receipt for the payment, as French law requires a supporting document for every
    /// accounting entry. Refused with 400 when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// The customer's subscription (`sub_…`) the credit belongs to: it pays only that
    /// subscription's charges and lapses when the subscription ends. It must not have ended,
    /// and its currency must be the grant's. Absent pays any of the customer's subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// PROMOTIONAL gives credit for free (a goodwill or marketing grant). PAID records credit
    /// the customer paid for outside of Meteroid: nothing is charged or invoiced, and the grant
    /// is shown as "Paid outside Meteroid". Usage paid with PAID currency credit is taxed on the
    /// invoice that bills it, so do not charge tax on the external sale of currency credit, or
    /// the customer is taxed twice. Credit in a credit unit is the exception: its usage is billed
    /// in credits, untaxed, so tax its sale as you would any other. Use a top-up to sell credit
    /// through Meteroid.
    pub r#type: CreditGrantType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateCreditGrantRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64, r#type: CreditGrantType) -> Self {
        Self {
            amount,
            applicable_product_ids: None,
            applies_to: None,
            cost: None,
            credit_unit: None,
            currency: None,
            expires_at: None,
            priority: None,
            reason: None,
            reference: None,
            subscription_id: None,
            r#type,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `applicable_product_ids`.
    #[must_use]
    pub fn applicable_product_ids(
        mut self,
        applicable_product_ids: impl Into<Vec<ProductId>>,
    ) -> Self {
        self.applicable_product_ids = Some(applicable_product_ids.into());
        self
    }

    /// Sets `applies_to`.
    #[must_use]
    pub fn applies_to(mut self, applies_to: impl Into<CreditAppliesTo>) -> Self {
        self.applies_to = Some(applies_to.into());
        self
    }

    /// Sets `cost`.
    #[must_use]
    pub fn cost(mut self, cost: impl Into<CreditCostRequest>) -> Self {
        self.cost = Some(cost.into());
        self
    }

    /// Sets `credit_unit`.
    #[must_use]
    pub fn credit_unit(mut self, credit_unit: impl Into<String>) -> Self {
        self.credit_unit = Some(credit_unit.into());
        self
    }

    /// Sets `currency`.
    #[must_use]
    pub fn currency(mut self, currency: impl Into<Currency>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, expires_at: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    /// Sets `priority`.
    #[must_use]
    pub fn priority(mut self, priority: impl Into<i32>) -> Self {
        self.priority = Some(priority.into());
        self
    }

    /// Sets `reason`.
    #[must_use]
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Sets `reference`.
    #[must_use]
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Sets `subscription_id`.
    #[must_use]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }
}
