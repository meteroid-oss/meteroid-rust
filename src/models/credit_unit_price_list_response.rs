// this file is @generated
use serde::{Deserialize, Serialize};

use super::credit_unit_price::CreditUnitPrice;

/// A credit unit's selling prices, newest first.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditUnitPriceListResponse {
    pub data: Vec<CreditUnitPrice>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnitPriceListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<CreditUnitPrice>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
