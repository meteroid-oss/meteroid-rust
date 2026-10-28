// this file is @generated
use serde::{Deserialize, Serialize};

use super::credit_expiry_unit::CreditExpiryUnit;

/// How long funded credit stays spendable. BILLING_PERIOD takes value 1.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditPackExpiry {
    /// DAY, MONTH or BILLING_PERIOD.
    pub unit: CreditExpiryUnit,

    /// How many days or months; 1 for BILLING_PERIOD.
    pub value: i32,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditPackExpiry {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(unit: CreditExpiryUnit, value: i32) -> Self {
        Self {
            unit,
            value,
            extra: serde_json::Map::new(),
        }
    }
}
