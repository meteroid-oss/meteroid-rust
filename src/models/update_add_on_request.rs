// this file is @generated
use serde::{Deserialize, Serialize};

use super::price_id::PriceId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAddOnRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub description: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub max_instances_per_subscription: Option<Option<i32>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub name: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub price_id: Option<Option<PriceId>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub self_serviceable: Option<Option<bool>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UpdateAddOnRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            max_instances_per_subscription: None,
            name: None,
            price_id: None,
            self_serviceable: None,
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

    /// Sets `max_instances_per_subscription`.
    #[must_use]
    pub fn max_instances_per_subscription(
        mut self,
        max_instances_per_subscription: impl Into<i32>,
    ) -> Self {
        self.max_instances_per_subscription = Some(Some(max_instances_per_subscription.into()));
        self
    }

    /// Sends `max_instances_per_subscription` as `null`, clearing it.
    #[must_use]
    pub fn clear_max_instances_per_subscription(mut self) -> Self {
        self.max_instances_per_subscription = Some(None);
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(Some(name.into()));
        self
    }

    /// Sends `name` as `null`, clearing it.
    #[must_use]
    pub fn clear_name(mut self) -> Self {
        self.name = Some(None);
        self
    }

    /// Sets `price_id`.
    #[must_use]
    pub fn price_id(mut self, price_id: impl Into<PriceId>) -> Self {
        self.price_id = Some(Some(price_id.into()));
        self
    }

    /// Sends `price_id` as `null`, clearing it.
    #[must_use]
    pub fn clear_price_id(mut self) -> Self {
        self.price_id = Some(None);
        self
    }

    /// Sets `self_serviceable`.
    #[must_use]
    pub fn self_serviceable(mut self, self_serviceable: impl Into<bool>) -> Self {
        self.self_serviceable = Some(Some(self_serviceable.into()));
        self
    }

    /// Sends `self_serviceable` as `null`, clearing it.
    #[must_use]
    pub fn clear_self_serviceable(mut self) -> Self {
        self.self_serviceable = Some(None);
        self
    }
}
