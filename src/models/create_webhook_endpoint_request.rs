// this file is @generated
use serde::{Deserialize, Serialize};

use super::webhook_header_input::WebhookHeaderInput;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateWebhookEndpointRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Event types to subscribe to. Omit or leave empty to receive every event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    /// Custom headers sent with every delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<WebhookHeaderInput>>,

    /// Deliveries started per second, at most (1 to 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_sec: Option<i32>,

    /// HTTPS destination. Private and loopback addresses are rejected unless the
    /// instance is configured to allow them.
    pub url: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateWebhookEndpointRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            description: None,
            event_types: None,
            headers: None,
            rate_limit_per_sec: None,
            url: url.into(),
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets `event_types`.
    #[must_use]
    pub fn event_types(mut self, event_types: impl Into<Vec<String>>) -> Self {
        self.event_types = Some(event_types.into());
        self
    }

    /// Sets `headers`.
    #[must_use]
    pub fn headers(mut self, headers: impl Into<Vec<WebhookHeaderInput>>) -> Self {
        self.headers = Some(headers.into());
        self
    }

    /// Sets `rate_limit_per_sec`.
    #[must_use]
    pub fn rate_limit_per_sec(mut self, rate_limit_per_sec: impl Into<i32>) -> Self {
        self.rate_limit_per_sec = Some(rate_limit_per_sec.into());
        self
    }
}
