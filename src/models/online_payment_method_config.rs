// this file is @generated
use serde::{Deserialize, Serialize};

use super::online_methods_config::OnlineMethodsConfig;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OnlinePaymentMethodConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OnlineMethodsConfig>,

    pub r#type: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl OnlinePaymentMethodConfig {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: None,
            r#type: "online".into(),
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `config`.
    #[must_use]
    pub fn config(mut self, config: impl Into<OnlineMethodsConfig>) -> Self {
        self.config = Some(config.into());
        self
    }
}

/// Sets `type` to `online`.
impl Default for OnlinePaymentMethodConfig {
    fn default() -> Self {
        Self {
            config: Default::default(),
            r#type: "online".into(),
            extra: serde_json::Map::new(),
        }
    }
}
