// this file is @generated
use serde::{Deserialize, Serialize};

use super::credit_pack_expiry::CreditPackExpiry;

/// Unspent included credit carried once into a new grant when it expires, spent before the next
/// period's credit.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditPackRollover {
    /// Term of the rolled credit from the original expiry: DAY or MONTH.
    pub expiry: CreditPackExpiry,

    /// Cap per pack, in the pack's credit; absent carries all that is left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<rust_decimal::Decimal>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditPackRollover {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(expiry: CreditPackExpiry) -> Self {
        Self {
            expiry,
            max_amount: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `max_amount`.
    #[must_use]
    pub fn max_amount(mut self, max_amount: impl Into<rust_decimal::Decimal>) -> Self {
        self.max_amount = Some(max_amount.into());
        self
    }
}
