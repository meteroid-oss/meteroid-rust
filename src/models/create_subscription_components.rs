// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    component_override::ComponentOverride, component_parameterization::ComponentParameterization,
    extra_component::ExtraComponent, price_component_id::PriceComponentId,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateSubscriptionComponents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_components: Option<Vec<ExtraComponent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_components: Option<Vec<ComponentOverride>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameterized_components: Option<Vec<ComponentParameterization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_components: Option<Vec<PriceComponentId>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateSubscriptionComponents {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            extra_components: None,
            overridden_components: None,
            parameterized_components: None,
            remove_components: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `extra_components`.
    #[must_use]
    pub fn extra_components(mut self, extra_components: impl Into<Vec<ExtraComponent>>) -> Self {
        self.extra_components = Some(extra_components.into());
        self
    }

    /// Sets `overridden_components`.
    #[must_use]
    pub fn overridden_components(
        mut self,
        overridden_components: impl Into<Vec<ComponentOverride>>,
    ) -> Self {
        self.overridden_components = Some(overridden_components.into());
        self
    }

    /// Sets `parameterized_components`.
    #[must_use]
    pub fn parameterized_components(
        mut self,
        parameterized_components: impl Into<Vec<ComponentParameterization>>,
    ) -> Self {
        self.parameterized_components = Some(parameterized_components.into());
        self
    }

    /// Sets `remove_components`.
    #[must_use]
    pub fn remove_components(
        mut self,
        remove_components: impl Into<Vec<PriceComponentId>>,
    ) -> Self {
        self.remove_components = Some(remove_components.into());
        self
    }
}
