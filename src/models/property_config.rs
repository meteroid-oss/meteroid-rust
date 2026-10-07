// this file is @generated
use serde::{Deserialize, Serialize};

use super::select_option::SelectOption;

/// Type-specific configuration. Only the fields relevant to `property_type` are interpreted.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PropertyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    /// Maximum length for `TEXT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,

    /// Inclusive numeric bounds for `NUMBER`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// Allowed choices for `SINGLE_SELECT` / `MULTI_SELECT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SelectOption>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl PropertyConfig {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            max: None,
            max_length: None,
            min: None,
            options: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `max`.
    #[must_use]
    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    /// Sets `max_length`.
    #[must_use]
    pub fn max_length(mut self, max_length: impl Into<i32>) -> Self {
        self.max_length = Some(max_length.into());
        self
    }

    /// Sets `min`.
    #[must_use]
    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    /// Sets `options`.
    #[must_use]
    pub fn options(mut self, options: impl Into<Vec<SelectOption>>) -> Self {
        self.options = Some(options.into());
        self
    }
}
