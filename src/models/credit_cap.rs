// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditCap {
    /// Usage beyond the credit invoiced per billing period, in minor units of `denomination`.
    pub amount: i64,

    /// A currency or credit unit code.
    pub denomination: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditCap {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64, denomination: impl Into<String>) -> Self {
        Self {
            amount,
            denomination: denomination.into(),
            extra: serde_json::Map::new(),
        }
    }
}
