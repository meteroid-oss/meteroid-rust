// this file is @generated
use serde::{Deserialize, Serialize};

/// How a usage line priced in a credit unit was paid: the credits it consumed, those the
/// customer's balance in the unit covered, and the rate the rest was billed at.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct InvoiceLineCreditUnit {
    /// The credit unit's code, e.g. `CREDITS`.
    pub code: String,

    /// Credits the usage priced.
    pub consumed: i64,

    /// Of which paid by the customer's credit wallet.
    pub covered: i64,

    /// Credit amounts are integers in 10^-precision credit.
    pub precision: i32,

    /// Invoice-currency major units per credit.
    pub rate: rust_decimal::Decimal,

    /// Credits per usage unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<rust_decimal::Decimal>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl InvoiceLineCreditUnit {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        consumed: i64,
        covered: i64,
        precision: i32,
        rate: rust_decimal::Decimal,
    ) -> Self {
        Self {
            code: code.into(),
            consumed,
            covered,
            precision,
            rate,
            unit_price: None,
            extra: serde_json::Map::new(),
        }
    }
}
