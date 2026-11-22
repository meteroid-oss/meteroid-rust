// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct InvoiceLineUsageBreakdownItem {
    /// Its part of the line's amount before discounts and tax, in minor units.
    pub amount_subtotal: i64,

    /// Its part of `credit_unit.consumed`, on a line priced in a credit unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits_consumed: Option<i64>,

    /// Set on the last item when there are more than 99 values: it sums the rest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others: Option<bool>,

    /// Usage of this value, in the units the line is priced in.
    pub quantity: rust_decimal::Decimal,

    /// Its part of the tax-included amount quoted, on a line billed from tax-inclusive prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quoted_amount_subtotal: Option<i64>,

    /// The property's value; absent for events without it, and for `others`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl InvoiceLineUsageBreakdownItem {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount_subtotal: i64, quantity: rust_decimal::Decimal) -> Self {
        Self {
            amount_subtotal,
            credits_consumed: None,
            others: None,
            quantity,
            quoted_amount_subtotal: None,
            value: None,
            extra: serde_json::Map::new(),
        }
    }
}
