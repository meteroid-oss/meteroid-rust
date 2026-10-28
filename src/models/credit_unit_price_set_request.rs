// this file is @generated
use serde::{Deserialize, Serialize};

/// The new price of one credit in a currency.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditUnitPriceSetRequest {
    /// Major units of the currency per credit, as a decimal string: positive, at most 12 decimal
    /// places.
    pub unit_price: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnitPriceSetRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(unit_price: impl Into<String>) -> Self {
        Self {
            unit_price: unit_price.into(),
            extra: serde_json::Map::new(),
        }
    }
}
