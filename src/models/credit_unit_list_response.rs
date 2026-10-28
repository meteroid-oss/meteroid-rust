// this file is @generated
use serde::{Deserialize, Serialize};

use super::credit_unit::CreditUnit;

/// The tenant's credit units.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditUnitListResponse {
    pub data: Vec<CreditUnit>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnitListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<CreditUnit>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
