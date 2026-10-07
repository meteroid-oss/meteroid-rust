// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PatchPlanRequest {
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
    pub name: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub self_service_rank: Option<Option<i32>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl PatchPlanRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            name: None,
            self_service_rank: None,
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

    /// Sets `self_service_rank`.
    #[must_use]
    pub fn self_service_rank(mut self, self_service_rank: impl Into<i32>) -> Self {
        self.self_service_rank = Some(Some(self_service_rank.into()));
        self
    }

    /// Sends `self_service_rank` as `null`, clearing it.
    #[must_use]
    pub fn clear_self_service_rank(mut self) -> Self {
        self.self_service_rank = Some(None);
        self
    }
}
