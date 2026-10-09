// this file is @generated
use serde::{Deserialize, Serialize};

use super::webhook_endpoint::WebhookEndpoint;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WebhookEndpointListResponse {
    pub data: Vec<WebhookEndpoint>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WebhookEndpointListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<WebhookEndpoint>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
