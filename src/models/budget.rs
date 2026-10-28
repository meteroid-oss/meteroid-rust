// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    budget_author::BudgetAuthor, budget_behavior::BudgetBehavior, budget_period::BudgetPeriod,
    currency::Currency,
};

/// A spending limit for one currency, set by you or by the customer, with alerts at percentages
/// of it. It counts the customer's whole spend, or one subscription's. A HARD budget also refuses
/// access once reached. A budget never charges the customer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Budget {
    /// In minor units of the currency or credit unit.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,

    /// When `estimated_spend` was last measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<chrono::DateTime<chrono::Utc>>,

    pub behavior: BudgetBehavior,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub created_by: BudgetAuthor,

    /// The credit unit (its code) whose usage the budget counts; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: amounts are integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Spend in the current billing period, including usage not invoiced yet, in minor units.
    /// Absent until first measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_spend: Option<i64>,

    /// Always false: the estimate prices usage that is not yet invoiced.
    pub exact: bool,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<BudgetPeriod>,

    /// The subscription whose spend the budget counts; null when it counts the whole customer's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// Percentages of the amount at which an alert is sent.
    pub thresholds: Vec<i32>,

    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// Send it back as `expected_version` when patching.
    pub version: i32,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Budget {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        behavior: BudgetBehavior,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: BudgetAuthor,
        exact: bool,
        id: impl Into<String>,
        thresholds: Vec<i32>,
        updated_at: chrono::DateTime<chrono::Utc>,
        version: i32,
    ) -> Self {
        Self {
            amount,
            archived_at: None,
            as_of: None,
            behavior,
            created_at,
            created_by,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            estimated_spend: None,
            exact,
            id: id.into(),
            period: None,
            subscription_id: None,
            thresholds,
            updated_at,
            version,
            extra: serde_json::Map::new(),
        }
    }
}
