// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_period_enum::BillingPeriodEnum, credit_pack_expiry::CreditPackExpiry,
    credit_pack_rollover::CreditPackRollover,
};

/// Credit pack: charged `unit_price` × `quantity` in advance each billing period, it grants
/// `credit_amount` × `quantity` of credit, in the plan currency or in `credit_unit_id`, once paid.
/// A zero `unit_price` makes it credit included with the plan (promotional). A currency pack
/// priced below its credit (e.g. 500.00 buys 550.00) grants the price paid as paid credit and the
/// difference as promotional bonus credit, which applies before tax.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditPlanFee {
    pub cadence: BillingPeriodEnum,

    pub credit_amount: rust_decimal::Decimal,

    /// The credit unit `credit_amount` is granted in; absent grants the plan currency. The pack is
    /// paid in the plan currency, and taxed when sold if it grants a credit unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<CreditPackExpiry>,

    /// When true, the pack is billed and its credit granted once, on the subscription's first
    /// invoice, instead of every period (e.g. a welcome credit). A plan change grants it on the
    /// next invoice only if the subscription never received a one-time credit of the same
    /// product; credit packs billed every period don't count. Cannot be combined with `rollover`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,

    pub quantity: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover: Option<CreditPackRollover>,

    pub r#type: String,

    pub unit_price: rust_decimal::Decimal,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditPlanFee {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        cadence: BillingPeriodEnum,
        credit_amount: rust_decimal::Decimal,
        quantity: i32,
        unit_price: rust_decimal::Decimal,
    ) -> Self {
        Self {
            cadence,
            credit_amount,
            credit_unit_id: None,
            expiry: None,
            once: None,
            quantity,
            rollover: None,
            r#type: "CREDIT".into(),
            unit_price,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `credit_unit_id`.
    #[must_use]
    pub fn credit_unit_id(mut self, credit_unit_id: impl Into<String>) -> Self {
        self.credit_unit_id = Some(credit_unit_id.into());
        self
    }

    /// Sets `expiry`.
    #[must_use]
    pub fn expiry(mut self, expiry: impl Into<CreditPackExpiry>) -> Self {
        self.expiry = Some(expiry.into());
        self
    }

    /// Sets `once`.
    #[must_use]
    pub fn once(mut self, once: impl Into<bool>) -> Self {
        self.once = Some(once.into());
        self
    }

    /// Sets `rollover`.
    #[must_use]
    pub fn rollover(mut self, rollover: impl Into<CreditPackRollover>) -> Self {
        self.rollover = Some(rollover.into());
        self
    }
}
