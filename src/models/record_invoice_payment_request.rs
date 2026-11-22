// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RecordInvoicePaymentRequest {
    /// The positive amount received, in minor units of the invoice currency (e.g. cents).
    pub amount: i64,

    /// When the payment was received, at most a day ahead. Defaults to the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Reference for the external payment, such as a bank transfer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl RecordInvoicePaymentRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            paid_at: None,
            reference: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `paid_at`.
    #[must_use]
    pub fn paid_at(mut self, paid_at: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.paid_at = Some(paid_at.into());
        self
    }

    /// Sets `reference`.
    #[must_use]
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }
}
