// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_applies_to::CreditAppliesTo, credit_cost::CreditCost, currency::Currency,
    ledger_entry_type::LedgerEntryType,
};

/// One movement of a customer's credit.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LedgerEntry {
    /// Signed change to the credit on account, in minor units.
    pub amount: i64,

    /// What the moved credit's grant pays: ALL any charge, USAGE only usage charges.
    pub applies_to: CreditAppliesTo,

    /// The credit on account after this entry, whatever it applies to.
    pub balance_after: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<CreditCost>,

    /// The grant the credit moved in or out of.
    pub credit_grant_id: String,

    /// The credit note that gave the credit back or took it back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<String>,

    /// The credit unit (its code) the amounts are in; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: amounts are integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// When the movement took effect; the ledger is ordered by it.
    pub effective_at: chrono::DateTime<chrono::Utc>,

    /// The entry's id (`wle_…`).
    pub id: String,

    /// The invoice the credit was applied to, bought on, or given back by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    /// Why the credit moved, as shown on the ledger.
    pub reason: String,

    /// When the movement was recorded.
    pub recorded_at: chrono::DateTime<chrono::Utc>,

    /// Your own reference, e.g. a contract or support ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Position within the currency's ledger in the order the movements took effect, from 1. A
    /// movement recorded later with an earlier date shifts the positions after it: identify an
    /// entry by its `id`.
    pub sequence: i64,

    /// The subscription the moved credit's grant belongs to; null when it pays any of the
    /// customer's subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// What moved the credit.
    pub r#type: LedgerEntryType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LedgerEntry {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        applies_to: CreditAppliesTo,
        balance_after: i64,
        credit_grant_id: impl Into<String>,
        effective_at: chrono::DateTime<chrono::Utc>,
        id: impl Into<String>,
        reason: impl Into<String>,
        recorded_at: chrono::DateTime<chrono::Utc>,
        sequence: i64,
        r#type: LedgerEntryType,
    ) -> Self {
        Self {
            amount,
            applies_to,
            balance_after,
            cost: None,
            credit_grant_id: credit_grant_id.into(),
            credit_note_id: None,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            effective_at,
            id: id.into(),
            invoice_id: None,
            reason: reason.into(),
            recorded_at,
            reference: None,
            sequence,
            subscription_id: None,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
