// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    add_on_id::AddOnId, subscription_add_on_customization::SubscriptionAddOnCustomization,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateSubscriptionAddOn {
    pub add_on_id: AddOnId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization: Option<SubscriptionAddOnCustomization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateSubscriptionAddOn {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(add_on_id: impl Into<AddOnId>) -> Self {
        Self {
            add_on_id: add_on_id.into(),
            customization: None,
            quantity: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `customization`.
    #[must_use]
    pub fn customization(
        mut self,
        customization: impl Into<SubscriptionAddOnCustomization>,
    ) -> Self {
        self.customization = Some(customization.into());
        self
    }

    /// Sets `quantity`.
    #[must_use]
    pub fn quantity(mut self, quantity: impl Into<i32>) -> Self {
        self.quantity = Some(quantity.into());
        self
    }
}
