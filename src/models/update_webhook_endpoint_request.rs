// this file is @generated
use serde::{Deserialize, Serialize};

use super::webhook_header_input::WebhookHeaderInput;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateWebhookEndpointRequest {
    /// Omit to leave unchanged; send `null` or an empty string to clear.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub description: Option<Option<String>>,

    /// Re-enabling an endpoint also resets its consecutive failure count.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub disabled: Option<Option<bool>>,

    /// Replaces the subscription list. An empty array subscribes to every event type.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub event_types: Option<Option<Vec<String>>>,

    /// Replaces the custom header list. An empty array removes every header.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub headers: Option<Option<Vec<WebhookHeaderInput>>>,

    /// Omit to leave unchanged; send `null` to remove the rate limit.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub rate_limit_per_sec: Option<Option<i32>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub url: Option<Option<String>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UpdateWebhookEndpointRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            disabled: None,
            event_types: None,
            headers: None,
            rate_limit_per_sec: None,
            url: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(Some(description.into()));
        self
    }

    /// Sends `description` as `null`, clearing it.
    #[must_use]
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// Sets `disabled`.
    #[must_use]
    pub fn disabled(mut self, disabled: impl Into<bool>) -> Self {
        self.disabled = Some(Some(disabled.into()));
        self
    }

    /// Sends `disabled` as `null`, clearing it.
    #[must_use]
    pub fn clear_disabled(mut self) -> Self {
        self.disabled = Some(None);
        self
    }

    /// Sets `event_types`.
    #[must_use]
    pub fn event_types(mut self, event_types: impl Into<Vec<String>>) -> Self {
        self.event_types = Some(Some(event_types.into()));
        self
    }

    /// Sends `event_types` as `null`, clearing it.
    #[must_use]
    pub fn clear_event_types(mut self) -> Self {
        self.event_types = Some(None);
        self
    }

    /// Sets `headers`.
    #[must_use]
    pub fn headers(mut self, headers: impl Into<Vec<WebhookHeaderInput>>) -> Self {
        self.headers = Some(Some(headers.into()));
        self
    }

    /// Sends `headers` as `null`, clearing it.
    #[must_use]
    pub fn clear_headers(mut self) -> Self {
        self.headers = Some(None);
        self
    }

    /// Sets `rate_limit_per_sec`.
    #[must_use]
    pub fn rate_limit_per_sec(mut self, rate_limit_per_sec: impl Into<i32>) -> Self {
        self.rate_limit_per_sec = Some(Some(rate_limit_per_sec.into()));
        self
    }

    /// Sends `rate_limit_per_sec` as `null`, clearing it.
    #[must_use]
    pub fn clear_rate_limit_per_sec(mut self) -> Self {
        self.rate_limit_per_sec = Some(None);
        self
    }

    /// Sets `url`.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(Some(url.into()));
        self
    }

    /// Sends `url` as `null`, clearing it.
    #[must_use]
    pub fn clear_url(mut self) -> Self {
        self.url = Some(None);
        self
    }
}
