// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_note_id::CreditNoteId, decline_kind::DeclineKind, invoice_id::InvoiceId,
    payment_status_enum::PaymentStatusEnum, payment_transaction_id::PaymentTransactionId,
    payment_type_enum::PaymentTypeEnum, refund_mode::RefundMode, reversal_kind::ReversalKind,
};

/// A refund row, or the payment that failed or was clawed back — the same shape the REST invoice
/// exposes as a transaction. `reversal_reason` is deliberately absent: it is free provider text and is
/// not carried on the outbox event.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct RefundEventData {
    pub amount: i64,

    pub amount_refunded: i64,

    pub amount_reversed: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<CreditNoteId>,

    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_kind: Option<DeclineKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<InvoiceId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_transaction_id: Option<PaymentTransactionId>,

    pub payment_type: PaymentTypeEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_transaction_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_mode: Option<RefundMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_kind: Option<ReversalKind>,

    pub status: PaymentStatusEnum,

    pub transaction_id: PaymentTransactionId,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl RefundEventData {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        amount_refunded: i64,
        amount_reversed: i64,
        currency: impl Into<String>,
        payment_type: PaymentTypeEnum,
        status: PaymentStatusEnum,
        transaction_id: PaymentTransactionId,
    ) -> Self {
        Self {
            amount,
            amount_refunded,
            amount_reversed,
            credit_note_id: None,
            currency: currency.into(),
            decline_kind: None,
            error: None,
            invoice_id: None,
            parent_transaction_id: None,
            payment_type,
            processed_at: None,
            provider_transaction_id: None,
            refund_mode: None,
            reversal_kind: None,
            status,
            transaction_id,
            extra: serde_json::Map::new(),
        }
    }
}
