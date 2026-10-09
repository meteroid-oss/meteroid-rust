// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    event_id::EventId, webhook_delivery_id::WebhookDeliveryId,
    webhook_delivery_status::WebhookDeliveryStatus, webhook_endpoint_id::WebhookEndpointId,
};

/// One event queued for one endpoint, with the state of its retry cycle.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WebhookDelivery {
    pub attempt_count: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub endpoint_id: WebhookEndpointId,

    pub event_type: String,

    pub id: WebhookDeliveryId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_response_status: Option<i32>,

    /// True when the delivery was created by a resend or a test event.
    pub manual: bool,

    pub message_id: EventId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt_at: Option<chrono::DateTime<chrono::Utc>>,

    pub status: WebhookDeliveryStatus,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WebhookDelivery {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        attempt_count: i32,
        created_at: chrono::DateTime<chrono::Utc>,
        endpoint_id: impl Into<WebhookEndpointId>,
        event_type: impl Into<String>,
        id: impl Into<WebhookDeliveryId>,
        manual: bool,
        message_id: impl Into<EventId>,
        status: WebhookDeliveryStatus,
    ) -> Self {
        Self {
            attempt_count,
            completed_at: None,
            created_at,
            endpoint_id: endpoint_id.into(),
            event_type: event_type.into(),
            id: id.into(),
            last_error: None,
            last_response_status: None,
            manual,
            message_id: message_id.into(),
            next_attempt_at: None,
            status,
            extra: serde_json::Map::new(),
        }
    }
}
