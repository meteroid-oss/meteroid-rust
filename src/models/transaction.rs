// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_note_id::CreditNoteId, customer_payment_method_id::CustomerPaymentMethodId,
    payment_method_info::PaymentMethodInfo, payment_status_enum::PaymentStatusEnum,
    payment_transaction_id::PaymentTransactionId, payment_type_enum::PaymentTypeEnum,
    refund_mode::RefundMode, reversal_kind::ReversalKind,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Transaction {
    pub amount: i64,

    /// On a PAYMENT: how much was voluntarily given back (the sum of its settled REFUND
    /// children). The invoice stays paid — the Refund credit note is what reduces it.
    pub amount_refunded: i64,

    /// On a PAYMENT: how much was involuntarily clawed back (chargeback, bank recall, lost
    /// dispute). This is what the invoice nets out, reopening it.
    pub amount_reversed: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<CreditNoteId>,

    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    pub id: PaymentTransactionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_transaction_id: Option<PaymentTransactionId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<CustomerPaymentMethodId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_info: Option<PaymentMethodInfo>,

    pub payment_type: PaymentTypeEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_transaction_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_mode: Option<RefundMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_kind: Option<ReversalKind>,

    /// The provider's raw cause, or what the operator typed when reversing by hand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_reason: Option<String>,

    pub status: PaymentStatusEnum,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Transaction {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        amount_refunded: i64,
        amount_reversed: i64,
        currency: impl Into<String>,
        id: PaymentTransactionId,
        payment_type: PaymentTypeEnum,
        status: PaymentStatusEnum,
    ) -> Self {
        Self {
            amount,
            amount_refunded,
            amount_reversed,
            credit_note_id: None,
            currency: currency.into(),
            error: None,
            id,
            parent_transaction_id: None,
            payment_method_id: None,
            payment_method_info: None,
            payment_type,
            processed_at: None,
            provider_transaction_id: None,
            refund_mode: None,
            reversal_kind: None,
            reversal_reason: None,
            status,
            extra: serde_json::Map::new(),
        }
    }
}
