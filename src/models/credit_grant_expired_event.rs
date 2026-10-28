// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_applies_to::CreditAppliesTo, credit_grant_id::CreditGrantId,
    credit_grant_lapse_reason::CreditGrantLapseReason, customer_id::CustomerId, event_id::EventId,
    event_type::EventType, ledger_entry_id::LedgerEntryId,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditGrantExpiredEvent {
    /// The products whose charges the grant pays; empty when it pays every product.
    pub applicable_product_ids: Vec<String>,

    /// What the grant pays: ALL any charge, USAGE only usage charges.
    pub applies_to: CreditAppliesTo,

    /// The grant that expired.
    pub credit_grant_id: CreditGrantId,

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

    /// Unspent credit that lapsed, in minor units of `currency` or `credit_unit`.
    pub expired_amount: i64,

    /// When the grant expired, or its subscription ended.
    pub expired_at: chrono::DateTime<chrono::Utc>,

    /// The credit ledger entry recording the expiry.
    pub ledger_entry_id: LedgerEntryId,

    /// SUBSCRIPTION_ENDED when the subscription the grant belongs to ended at or before the grant's
    /// expiry, EXPIRED otherwise.
    pub reason: CreditGrantLapseReason,

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

impl CreditGrantExpiredEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        applicable_product_ids: Vec<String>,
        applies_to: CreditAppliesTo,
        credit_grant_id: impl Into<CreditGrantId>,
        customer_id: impl Into<CustomerId>,
        expired_amount: i64,
        expired_at: chrono::DateTime<chrono::Utc>,
        ledger_entry_id: impl Into<LedgerEntryId>,
        reason: CreditGrantLapseReason,
        id: impl Into<EventId>,
        timestamp: chrono::DateTime<chrono::Utc>,
        r#type: EventType,
    ) -> Self {
        Self {
            applicable_product_ids,
            applies_to,
            credit_grant_id: credit_grant_id.into(),
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            customer_id: customer_id.into(),
            expired_amount,
            expired_at,
            ledger_entry_id: ledger_entry_id.into(),
            reason,
            subscription_id: None,
            id: id.into(),
            timestamp,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
