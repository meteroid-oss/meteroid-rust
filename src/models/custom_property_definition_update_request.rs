// this file is @generated
use serde::{Deserialize, Serialize};

use super::property_config::PropertyConfig;

/// Update of a definition. `key`, `entity_type` and `property_type` are immutable and cannot be
/// changed here. Any field left absent is unchanged.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomPropertyDefinitionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<PropertyConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CustomPropertyDefinitionUpdateRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: None,
            default_value: None,
            description: None,
            display_order: None,
            name: None,
            required: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `config`.
    #[must_use]
    pub fn config(mut self, config: impl Into<PropertyConfig>) -> Self {
        self.config = Some(config.into());
        self
    }

    /// Sets `default_value`.
    #[must_use]
    pub fn default_value(mut self, default_value: impl Into<serde_json::Value>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets `display_order`.
    #[must_use]
    pub fn display_order(mut self, display_order: impl Into<i32>) -> Self {
        self.display_order = Some(display_order.into());
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets `required`.
    #[must_use]
    pub fn required(mut self, required: impl Into<bool>) -> Self {
        self.required = Some(required.into());
        self
    }
}
