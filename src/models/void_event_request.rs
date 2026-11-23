// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VoidEventRequest {
    /// External customer alias of the event. Set this or `customer_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,

    /// Meteroid customer ID of the event. Set this or `customer_alias`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl VoidEventRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            customer_alias: None,
            customer_id: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `customer_alias`.
    #[must_use]
    pub fn customer_alias(mut self, customer_alias: impl Into<String>) -> Self {
        self.customer_alias = Some(customer_alias.into());
        self
    }

    /// Sets `customer_id`.
    #[must_use]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }
}
