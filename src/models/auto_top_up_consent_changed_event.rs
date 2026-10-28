// this file is @generated
use serde::{Deserialize, Serialize};

use super::{actor::Actor, customer_id::CustomerId, event_id::EventId, event_type::EventType};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AutoTopUpConsentChangedEvent {
    /// Who gave, changed or withdrew the authorization.
    pub actor: Actor,

    /// When the authorization changed.
    pub changed_at: chrono::DateTime<chrono::Utc>,

    /// Identifies the terms authorized now, without disclosing payment details; absent when
    /// withdrawn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_revision: Option<String>,

    /// When that attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_attested_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_attested_by: Option<Actor>,

    /// The wallet's currency (ISO 4217) or credit unit code.
    pub currency: String,

    pub customer_id: CustomerId,

    /// Whether automatic top-up is authorized after this change.
    pub enabled: bool,

    /// Identifies the terms authorized before the change; absent when none were.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_configuration_revision: Option<String>,

    pub id: EventId,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub r#type: EventType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AutoTopUpConsentChangedEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        actor: Actor,
        changed_at: chrono::DateTime<chrono::Utc>,
        currency: impl Into<String>,
        customer_id: impl Into<CustomerId>,
        enabled: bool,
        id: impl Into<EventId>,
        timestamp: chrono::DateTime<chrono::Utc>,
        r#type: EventType,
    ) -> Self {
        Self {
            actor,
            changed_at,
            configuration_revision: None,
            consent_attested_at: None,
            consent_attested_by: None,
            currency: currency.into(),
            customer_id: customer_id.into(),
            enabled,
            previous_configuration_revision: None,
            id: id.into(),
            timestamp,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
