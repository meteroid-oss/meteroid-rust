// this file is @generated
use serde::{Deserialize, Serialize};

use super::budget_behavior::BudgetBehavior;

/// The currency or credit unit, and the subscription, are fixed at creation.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PatchBudgetRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub amount: Option<Option<i64>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub behavior: Option<Option<BudgetBehavior>>,

    /// The `version` you read; a newer stored version is refused with 409.
    pub expected_version: i32,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub thresholds: Option<Option<Vec<i32>>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl PatchBudgetRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(expected_version: i32) -> Self {
        Self {
            amount: None,
            behavior: None,
            expected_version,
            thresholds: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `amount`.
    #[must_use]
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.amount = Some(Some(amount.into()));
        self
    }

    /// Sends `amount` as `null`, clearing it.
    #[must_use]
    pub fn clear_amount(mut self) -> Self {
        self.amount = Some(None);
        self
    }

    /// Sets `behavior`.
    #[must_use]
    pub fn behavior(mut self, behavior: impl Into<BudgetBehavior>) -> Self {
        self.behavior = Some(Some(behavior.into()));
        self
    }

    /// Sends `behavior` as `null`, clearing it.
    #[must_use]
    pub fn clear_behavior(mut self) -> Self {
        self.behavior = Some(None);
        self
    }

    /// Sets `thresholds`.
    #[must_use]
    pub fn thresholds(mut self, thresholds: impl Into<Vec<i32>>) -> Self {
        self.thresholds = Some(Some(thresholds.into()));
        self
    }

    /// Sends `thresholds` as `null`, clearing it.
    #[must_use]
    pub fn clear_thresholds(mut self) -> Self {
        self.thresholds = Some(None);
        self
    }
}
