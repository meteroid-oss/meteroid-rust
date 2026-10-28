// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_note_id::CreditNoteId, customer_id::CustomerId, invoice_id::InvoiceId,
    payment_transaction_id::PaymentTransactionId,
};

/// Purchased credit was already spent when its money came back.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletCreditUnrecoveredEventData {
    /// `PAYMENT_DISPUTED` (lost to a dispute or chargeback), `PAYMENT_REVERSED` (a failed
    /// collection clawed back), `PAYMENT_REFUNDED` (a payment refunded without a credit note, at
    /// the provider or as an overpayment, after the credit it backed was spent) or
    /// `CREDIT_NOTE_REFUNDED`.
    pub cause: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<CreditNoteId>,

    /// Unspent credit taken back from the wallet at the same time, in minor units.
    pub credit_taken_back: i64,

    /// The credit unit (its code) the amounts are in; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: amounts are integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    /// Spent credit this event adds to the loss, in minor units. A failed collection reopens its
    /// invoice, which collects it again; a dispute's loss stays unless the dispute is won.
    /// `wallet.credit_recovered` reports what comes back.
    pub credit_unrecovered: i64,

    /// The loss of this money flow (an invoice's payments, a credit note, or a payment) once
    /// this event applies.
    pub credit_unrecovered_total: i64,

    /// Absent when the amounts are in a credit unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    pub customer_id: CustomerId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<InvoiceId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentTransactionId>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletCreditUnrecoveredEventData {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        cause: impl Into<String>,
        credit_taken_back: i64,
        credit_unrecovered: i64,
        credit_unrecovered_total: i64,
        customer_id: impl Into<CustomerId>,
    ) -> Self {
        Self {
            cause: cause.into(),
            credit_note_id: None,
            credit_taken_back,
            credit_unit: None,
            credit_unit_precision: None,
            credit_unrecovered,
            credit_unrecovered_total,
            currency: None,
            customer_id: customer_id.into(),
            invoice_id: None,
            payment_id: None,
            extra: serde_json::Map::new(),
        }
    }
}
