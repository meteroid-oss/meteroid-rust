// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    actor::Actor, credit_applies_to::CreditAppliesTo, credit_grant_id::CreditGrantId,
    credit_note_id::CreditNoteId, customer_id::CustomerId, event_id::EventId,
    event_type::EventType, invoice_id::InvoiceId, ledger_entry_id::LedgerEntryId,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletDebitedEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Actor>,

    /// In minor units of `currency` or `credit_unit`.
    pub amount: i64,

    /// The products whose charges the grant pays; empty when it pays every product.
    pub applicable_product_ids: Vec<String>,

    /// What the grant pays: ALL any charge, USAGE only usage charges.
    pub applies_to: CreditAppliesTo,

    /// The grant the credit left.
    pub credit_grant_id: CreditGrantId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<CreditNoteId>,

    /// The credit unit (its code) the amounts are in; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: amounts are integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    /// Absent when the amounts are in a credit unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    pub customer_id: CustomerId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<InvoiceId>,

    /// The credit ledger entry recording the movement.
    pub ledger_entry_id: LedgerEntryId,

    /// Why the credit moved, as shown on the ledger.
    pub reason: String,

    /// The subscription the grant belongs to: it pays only that subscription's charges. Null when
    /// it pays any of the customer's subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    pub id: EventId,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub r#type: EventType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletDebitedEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        applicable_product_ids: Vec<String>,
        applies_to: CreditAppliesTo,
        credit_grant_id: impl Into<CreditGrantId>,
        customer_id: impl Into<CustomerId>,
        ledger_entry_id: impl Into<LedgerEntryId>,
        reason: impl Into<String>,
        id: impl Into<EventId>,
        timestamp: chrono::DateTime<chrono::Utc>,
        r#type: EventType,
    ) -> Self {
        Self {
            actor: None,
            amount,
            applicable_product_ids,
            applies_to,
            credit_grant_id: credit_grant_id.into(),
            credit_note_id: None,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            customer_id: customer_id.into(),
            invoice_id: None,
            ledger_entry_id: ledger_entry_id.into(),
            reason: reason.into(),
            subscription_id: None,
            id: id.into(),
            timestamp,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
