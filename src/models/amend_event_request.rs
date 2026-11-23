// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AmendEventRequest {
    /// Allow a timestamp more than 1 day in the past. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_backfilling: Option<bool>,

    /// External customer alias of the event. Set this or `customer_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,

    /// Meteroid customer ID of the event. Set this or `customer_alias`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    /// The corrected properties. They replace the stored ones.
    pub properties: std::collections::HashMap<String, String>,

    /// RFC 3339 timestamp of the corrected event.
    /// Must be between 24 hours ago and 1 hour from now. Set `allow_backfilling` to remove the past limit.
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AmendEventRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        properties: std::collections::HashMap<String, String>,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            allow_backfilling: None,
            customer_alias: None,
            customer_id: None,
            properties,
            timestamp,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `allow_backfilling`.
    #[must_use]
    pub fn allow_backfilling(mut self, allow_backfilling: impl Into<bool>) -> Self {
        self.allow_backfilling = Some(allow_backfilling.into());
        self
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
