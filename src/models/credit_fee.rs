// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_pack_expiry::CreditPackExpiry, credit_pack_rollover::CreditPackRollover,
    credit_pack_unit::CreditPackUnit,
};

/// A credit pack: charged like a fixed fee, it grants `credit_amount` × `quantity` of credit once
/// paid, in the subscription currency or in `credit_unit`. A zero `unit_price` makes it credit
/// included with the plan (promotional). A currency pack priced below its credit grants the price
/// paid as paid credit and the difference as promotional bonus credit, which applies before tax.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditFee {
    /// Credit granted per pack, in major units of the subscription currency, or a number of
    /// credits of `credit_unit` when set.
    pub credit_amount: rust_decimal::Decimal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<CreditPackUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<CreditPackExpiry>,

    /// When true, billed and granted once, on the subscription's first invoice, instead of every
    /// period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,

    /// Packs billed and granted each time.
    pub quantity: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover: Option<CreditPackRollover>,

    pub r#type: String,

    /// Price of one pack, in major units of the subscription currency.
    pub unit_price: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditFee {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        credit_amount: rust_decimal::Decimal,
        quantity: i32,
        unit_price: rust_decimal::Decimal,
    ) -> Self {
        Self {
            credit_amount,
            credit_unit: None,
            expiry: None,
            once: None,
            quantity,
            rollover: None,
            r#type: "CREDIT".into(),
            unit_price,
            extra: serde_json::Map::new(),
        }
    }
}
