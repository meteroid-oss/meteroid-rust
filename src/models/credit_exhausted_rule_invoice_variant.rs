// this file is @generated
use serde::{Deserialize, Serialize};

/// Usage beyond the credit is invoiced.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditExhaustedRuleInvoiceVariant {
    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditExhaustedRuleInvoiceVariant {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r#type: "invoice".into(),
            extra: serde_json::Map::new(),
        }
    }
}

/// Sets `type` to `invoice`.
impl Default for CreditExhaustedRuleInvoiceVariant {
    fn default() -> Self {
        Self {
            r#type: "invoice".into(),
            extra: serde_json::Map::new(),
        }
    }
}
