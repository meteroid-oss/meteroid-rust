// this file is @generated
use serde::{Deserialize, Serialize};

use super::{auto_top_up_method::AutoTopUpMethod, auto_top_up_status::AutoTopUpStatus};

/// Automatic top-up of the credit that applies to ALL charges: it charges the customer's payment
/// method when the balance falls below `threshold`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AutoTopUp {
    /// FIXED only: credit bought each time, in minor units of the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// When an operator attested that the customer agreed to automatic charges on exactly these
    /// terms. Absent when the customer confirmed them in their portal, or the terms changed since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_consent_attested_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Most charged in a rolling day, tax included. For a credit unit wallet caps are money, in
    /// minor units of the customer's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<i64>,

    /// Why the last charge failed; may carry payment provider detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,

    /// The last charge that succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_succeeded_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Largest single charge, in the same money as `minimum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    /// FIXED buys `amount` each time; TO_TARGET buys back up to `target`.
    pub method: AutoTopUpMethod,

    /// Smallest single charge. For a credit unit wallet it is money (the price before tax, in
    /// minor units of the customer's currency), like the caps; the threshold, amount and target
    /// are credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,

    /// Most charged in a calendar month (UTC), tax included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_cap: Option<i64>,

    /// Absent charges the customer's default payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,

    /// ACTIVE, DISABLED (terms kept) or SUSPENDED after repeated failed charges.
    pub status: AutoTopUpStatus,

    /// Set while SUSPENDED: since when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,

    /// TO_TARGET only: the balance a top-up brings the wallet back to, in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<i64>,

    /// Tops up once the balance is at or below this, in minor units of the wallet.
    pub threshold: i64,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AutoTopUp {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(method: AutoTopUpMethod, status: AutoTopUpStatus, threshold: i64) -> Self {
        Self {
            amount: None,
            customer_consent_attested_at: None,
            daily_cap: None,
            last_error: None,
            last_succeeded_at: None,
            maximum: None,
            method,
            minimum: None,
            monthly_cap: None,
            payment_method_id: None,
            status,
            suspended_at: None,
            target: None,
            threshold,
            extra: serde_json::Map::new(),
        }
    }
}
