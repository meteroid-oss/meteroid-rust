// this file is @generated
use serde::{Deserialize, Serialize};

use super::credit_unit_price::CreditUnitPrice;

/// A tenant-defined denomination usage can be priced or credit granted in, instead of money.
/// `code` and `precision` are immutable once created.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditUnit {
    /// Archived units take no new prices, packs or grants; existing ones keep working.
    pub archived: bool,

    /// Upper-case code naming the unit in wallets, grants and webhooks, e.g. `CREDITS`.
    pub code: String,

    /// The unit's id (`cun_…`).
    pub id: String,

    /// Singular display name, e.g. "Credit".
    pub name: String,

    /// Plural display name, e.g. "Credits".
    pub plural_name: String,

    /// Minor-unit exponent: amounts of this unit are integers of 10^-precision credit.
    pub precision: i32,

    /// The active selling prices, one per currency.
    pub prices: Vec<CreditUnitPrice>,

    /// The product sales of this unit are booked under: their tax category and accounting code.
    /// The unit can be sold (top-ups) only once it is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,

    /// Short display symbol, e.g. "cr".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnit {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        archived: bool,
        code: impl Into<String>,
        id: impl Into<String>,
        name: impl Into<String>,
        plural_name: impl Into<String>,
        precision: i32,
        prices: Vec<CreditUnitPrice>,
    ) -> Self {
        Self {
            archived,
            code: code.into(),
            id: id.into(),
            name: name.into(),
            plural_name: plural_name.into(),
            precision,
            prices,
            product_id: None,
            symbol: None,
            extra: serde_json::Map::new(),
        }
    }
}
