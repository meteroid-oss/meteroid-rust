// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    webhook_endpoint_disabled_reason::WebhookEndpointDisabledReason,
    webhook_endpoint_id::WebhookEndpointId, webhook_header::WebhookHeader,
};

/// A destination Meteroid POSTs signed event payloads to.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WebhookEndpoint {
    /// Failures since the last success, reset to 0 on any 2xx.
    pub consecutive_failures: i32,

    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub disabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<WebhookEndpointDisabledReason>,

    /// Subscribed event types. Empty means every event type.
    pub event_types: Vec<String>,

    /// Custom headers sent with every delivery.
    pub headers: Vec<WebhookHeader>,

    pub id: WebhookEndpointId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_success_at: Option<chrono::DateTime<chrono::Utc>>,

    /// How many deliveries this endpoint may have in flight at once. Read-only; it is
    /// set from the tenant's environment when the endpoint is created.
    pub max_in_flight: i32,

    /// A sensitive header still waits for its value; the endpoint cannot be enabled
    /// until it is set.
    pub needs_setup: bool,

    /// The endpoint was unreachable several times in a row: nothing is sent before
    /// this time, then it is retried one delivery at a time until it answers again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_until: Option<chrono::DateTime<chrono::Utc>>,

    /// Deliveries started per second, at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_sec: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    pub url: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WebhookEndpoint {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        consecutive_failures: i32,
        created_at: chrono::DateTime<chrono::Utc>,
        disabled: bool,
        event_types: Vec<String>,
        headers: Vec<WebhookHeader>,
        id: impl Into<WebhookEndpointId>,
        max_in_flight: i32,
        needs_setup: bool,
        url: impl Into<String>,
    ) -> Self {
        Self {
            consecutive_failures,
            created_at,
            description: None,
            disabled,
            disabled_reason: None,
            event_types,
            headers,
            id: id.into(),
            last_failure_at: None,
            last_success_at: None,
            max_in_flight,
            needs_setup,
            paused_until: None,
            rate_limit_per_sec: None,
            updated_at: None,
            url: url.into(),
            extra: serde_json::Map::new(),
        }
    }
}
