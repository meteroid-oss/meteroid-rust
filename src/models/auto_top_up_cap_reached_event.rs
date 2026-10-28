// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer_id::CustomerId, event_id::EventId, event_type::EventType};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AutoTopUpCapReachedEvent {
    /// What the suppressed top-up would have charged, tax included once it was quoted (before
    /// tax when the cap stopped it before then). Caps are always money: for a credit unit wallet,
    /// `currency` is the customer's currency.
    pub attempted_amount: i64,

    /// The cap, tax included, in minor units of `currency`.
    pub cap: i64,

    /// The currency the cap and the charge are in (ISO 4217).
    pub currency: String,

    pub customer_id: CustomerId,

    /// `DAILY` (rolling day) or `MONTHLY` (calendar month, UTC).
    pub period: String,

    /// End of the capped period.
    pub period_end: chrono::DateTime<chrono::Utc>,

    /// Start of the capped period.
    pub period_start: chrono::DateTime<chrono::Utc>,

    pub id: EventId,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub r#type: EventType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AutoTopUpCapReachedEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        attempted_amount: i64,
        cap: i64,
        currency: impl Into<String>,
        customer_id: impl Into<CustomerId>,
        period: impl Into<String>,
        period_end: chrono::DateTime<chrono::Utc>,
        period_start: chrono::DateTime<chrono::Utc>,
        id: impl Into<EventId>,
        timestamp: chrono::DateTime<chrono::Utc>,
        r#type: EventType,
    ) -> Self {
        Self {
            attempted_amount,
            cap,
            currency: currency.into(),
            customer_id: customer_id.into(),
            period: period.into(),
            period_end,
            period_start,
            id: id.into(),
            timestamp,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
