// this file is @generated
use serde::{Deserialize, Serialize};

/// The credit unit a usage fee's amounts are expressed in, with its value in the subscription
/// currency.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct FeeCreditUnit {
    /// The credit unit's code, e.g. `CREDITS`.
    pub code: String,

    /// The credit unit's id (`cun_…`).
    pub credit_unit_id: String,

    /// Decimal places of the unit: credit balances and ledger amounts in it are integers of
    /// 10^-precision credit.
    pub precision: i32,

    /// Subscription-currency major units per credit: the money billed, before tax, for each
    /// credit the customer's balance in the unit does not cover.
    pub rate: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl FeeCreditUnit {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        credit_unit_id: impl Into<String>,
        precision: i32,
        rate: rust_decimal::Decimal,
    ) -> Self {
        Self {
            code: code.into(),
            credit_unit_id: credit_unit_id.into(),
            precision,
            rate,
            extra: serde_json::Map::new(),
        }
    }
}
