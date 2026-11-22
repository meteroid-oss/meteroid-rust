// this file is @generated
use serde::{Deserialize, Serialize};

use super::reversal_kind::ReversalKind;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReverseInvoicePaymentRequest {
    /// `CHARGEBACK`, `DEBTOR_RECALL` and `DISPUTE` leave the invoice paid and mark the payment
    /// disputed. Any other kind makes the amount due again.
    pub kind: ReversalKind,

    /// Additional details about the reversal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// When the reversal occurred, at most a day ahead. Defaults to the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl ReverseInvoicePaymentRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(kind: ReversalKind) -> Self {
        Self {
            kind,
            reason: None,
            reversed_at: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `reason`.
    #[must_use]
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Sets `reversed_at`.
    #[must_use]
    pub fn reversed_at(mut self, reversed_at: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.reversed_at = Some(reversed_at.into());
        self
    }
}
