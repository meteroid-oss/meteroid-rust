// this file is @generated
use serde::{Deserialize, Serialize};

use super::credit_cap::CreditCap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditCaps {
    /// One per denomination; usage charged in a denomination without a cap pauses at zero.
    pub caps: Vec<CreditCap>,

    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditCaps {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(caps: Vec<CreditCap>) -> Self {
        Self {
            caps,
            r#type: "invoice_up_to".into(),
            extra: serde_json::Map::new(),
        }
    }
}
