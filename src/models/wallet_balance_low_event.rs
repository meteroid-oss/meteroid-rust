// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer_id::CustomerId, event_id::EventId, event_type::EventType};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletBalanceLowEvent {
    /// The balance when it fell below `threshold`, in minor units: the wallet's `balance`, all
    /// its credit on account net of the unbilled usage it pays, including credit that belongs to
    /// a subscription or is limited to usage or products. Sent once per crossing, for the lowest
    /// threshold crossed; it re-arms once the balance is back at the threshold. Because that
    /// credit counts here and not in `wallet.depleted`, a wallet can be depleted without this
    /// event firing.
    pub balance: i64,

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

    /// The part of `balance` that pays any charge of any subscription: what automatic top-ups
    /// refill and `wallet.depleted` watches. Absent on events recorded before it was reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_balance: Option<i64>,

    /// The low-balance threshold crossed, in the same minor units.
    pub threshold: i64,

    pub id: EventId,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub r#type: EventType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletBalanceLowEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        balance: i64,
        customer_id: impl Into<CustomerId>,
        threshold: i64,
        id: impl Into<EventId>,
        timestamp: chrono::DateTime<chrono::Utc>,
        r#type: EventType,
    ) -> Self {
        Self {
            balance,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            customer_id: customer_id.into(),
            shared_balance: None,
            threshold,
            id: id.into(),
            timestamp,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
