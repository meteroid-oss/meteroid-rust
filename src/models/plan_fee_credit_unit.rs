// this file is @generated
use serde::{Deserialize, Serialize};

/// The credit unit a plan's usage price is expressed in, with its value in the plan currency.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PlanFeeCreditUnit {
    /// An active credit unit of the tenant (`GET /api/v1/credit-units`).
    pub credit_unit_id: String,

    /// Plan-currency major units per credit (e.g. "0.01"), positive: the money billed for each
    /// credit the customer's balance in the unit does not cover, before tax.
    pub rate: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl PlanFeeCreditUnit {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(credit_unit_id: impl Into<String>, rate: rust_decimal::Decimal) -> Self {
        Self {
            credit_unit_id: credit_unit_id.into(),
            rate,
            extra: serde_json::Map::new(),
        }
    }
}
