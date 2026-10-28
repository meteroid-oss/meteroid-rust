// this file is @generated
use serde::{Deserialize, Serialize};

use super::usage_cost_unit::UsageCostUnit;

/// Usage priced as its invoice line prices it, before discounts and credits: tax-inclusive when
/// the subscription's prices are.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UsageCost {
    pub amount: rust_decimal::Decimal,

    /// A currency code, or a credit unit code when `unit` is `CREDIT_UNIT`.
    pub currency: String,

    pub unit: UsageCostUnit,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageCost {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: rust_decimal::Decimal,
        currency: impl Into<String>,
        unit: UsageCostUnit,
    ) -> Self {
        Self {
            amount,
            currency: currency.into(),
            unit,
            extra: serde_json::Map::new(),
        }
    }
}
