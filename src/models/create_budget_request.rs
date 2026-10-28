// this file is @generated
use serde::{Deserialize, Serialize};

use super::{budget_behavior::BudgetBehavior, currency::Currency};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateBudgetRequest {
    /// In minor units of the currency or credit unit.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<BudgetBehavior>,

    /// A credit unit (its code): the budget counts the usage priced in that unit, in its minor
    /// units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// An active subscription of the customer (`sub_…`): the budget then counts only that
    /// subscription's spend, in its currency. Absent counts the whole customer's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// Percentages of the amount (1 to 100, at most 10). Defaults to 50, 80 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<Vec<i32>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateBudgetRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            behavior: None,
            credit_unit: None,
            currency: None,
            subscription_id: None,
            thresholds: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `behavior`.
    #[must_use]
    pub fn behavior(mut self, behavior: impl Into<BudgetBehavior>) -> Self {
        self.behavior = Some(behavior.into());
        self
    }

    /// Sets `credit_unit`.
    #[must_use]
    pub fn credit_unit(mut self, credit_unit: impl Into<String>) -> Self {
        self.credit_unit = Some(credit_unit.into());
        self
    }

    /// Sets `currency`.
    #[must_use]
    pub fn currency(mut self, currency: impl Into<Currency>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Sets `subscription_id`.
    #[must_use]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    /// Sets `thresholds`.
    #[must_use]
    pub fn thresholds(mut self, thresholds: impl Into<Vec<i32>>) -> Self {
        self.thresholds = Some(thresholds.into());
        self
    }
}
