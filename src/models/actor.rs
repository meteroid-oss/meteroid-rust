// this file is @generated
use serde::{Deserialize, Serialize};

use super::actor_type::ActorType;

/// Who made a change.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Actor {
    /// The user's id (`usr_…`) for USER, the customer's (`cus_…`) for CUSTOMER, the API key's
    /// or app's for API_KEY and OAUTH_APP; absent for SYSTEM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// USER, CUSTOMER, API_KEY, OAUTH_APP or SYSTEM.
    pub r#type: ActorType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Actor {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(r#type: ActorType) -> Self {
        Self {
            id: None,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
