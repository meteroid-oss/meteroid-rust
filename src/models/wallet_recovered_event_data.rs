// this file is @generated
use serde::{Deserialize, Serialize};

use super::customer_id::CustomerId;

/// A wallet's balance (credit that pays any charge of any of the customer's subscriptions, net of
/// unbilled usage) came back above zero.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletRecoveredEventData {
    /// The balance (credit on account that pays any charge of any of the customer's subscriptions,
    /// net of unbilled usage) now that it is back above zero, in
    /// minor units.
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

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletRecoveredEventData {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(balance: i64, customer_id: impl Into<CustomerId>) -> Self {
        Self {
            balance,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            customer_id: customer_id.into(),
            extra: serde_json::Map::new(),
        }
    }
}
