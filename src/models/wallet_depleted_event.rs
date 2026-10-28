// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer_id::CustomerId, event_id::EventId, event_type::EventType};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletDepletedEvent {
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

    pub id: EventId,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub r#type: EventType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletDepletedEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        customer_id: impl Into<CustomerId>,
        id: impl Into<EventId>,
        timestamp: chrono::DateTime<chrono::Utc>,
        r#type: EventType,
    ) -> Self {
        Self {
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            customer_id: customer_id.into(),
            id: id.into(),
            timestamp,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
