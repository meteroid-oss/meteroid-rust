// this file is @generated
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{
    credit_caps::CreditCaps,
    credit_exhausted_rule_invoice_variant::CreditExhaustedRuleInvoiceVariant,
    credit_exhausted_rule_pause_variant::CreditExhaustedRulePauseVariant,
};
use crate::models::codec;

/// What usage does once the credit it draws on runs out, for every denomination (currency or
/// credit unit) the plan charges usage in. A customer's wallet can override it.
///
/// Variants are told apart by `type`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CreditExhaustedRule {
    Invoice(CreditExhaustedRuleInvoiceVariant),
    Pause(CreditExhaustedRulePauseVariant),
    InvoiceUpTo(CreditCaps),
    /// A variant this version of the SDK does not know yet, as received.
    Unknown(serde_json::Value),
}

impl CreditExhaustedRule {
    /// The `type` value of this variant.
    #[must_use]
    pub fn tag(&self) -> Option<&str> {
        match self {
            Self::Invoice(_) => Some("invoice"),
            Self::Pause(_) => Some("pause"),
            Self::InvoiceUpTo(_) => Some("invoice_up_to"),
            Self::Unknown(value) => value.get("type")?.as_str(),
        }
    }
}

impl Serialize for CreditExhaustedRule {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Invoice(value) => codec::internally_tagged(serializer, "type", "invoice", value),
            Self::Pause(value) => codec::internally_tagged(serializer, "type", "pause", value),
            Self::InvoiceUpTo(value) => {
                codec::internally_tagged(serializer, "type", "invoice_up_to", value)
            }
            Self::Unknown(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CreditExhaustedRule {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (tag, value) = codec::split_tag(deserializer, "type")?;
        Ok(match tag.as_str() {
            "invoice" => Self::Invoice(codec::from_value(value)?),
            "pause" => Self::Pause(codec::from_value(value)?),
            "invoice_up_to" => Self::InvoiceUpTo(codec::from_value(value)?),
            _ => Self::Unknown(value),
        })
    }
}
