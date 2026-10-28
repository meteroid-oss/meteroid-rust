// this file is @generated
use serde::{Deserialize, Serialize};

use super::currency::Currency;

/// Money in minor units of `currency`, net of tax.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditCostRequest {
    /// In minor units of `currency`; zero or positive.
    pub amount: i64,

    pub currency: Currency,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditCostRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self {
            amount,
            currency,
            extra: serde_json::Map::new(),
        }
    }
}
