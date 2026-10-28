// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The group a grant is shown in, from where it came from and what it pays:
/// - INCLUDED: credit a subscription's plan includes, which renews with it;
/// - PREPAID: credit bought (purchases, top-ups, paid packs, credit paid for outside Meteroid);
/// - BONUS: credit given (the bonus part of discounted credit, promo codes, welcome credit);
/// - ACCOUNT_BALANCE: the customer's money that pays any invoice (credit notes, returned invoices,
///   the balance carried from before credit grants, credit added for any charge).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditGrantKind {
    Included,
    Prepaid,
    Bonus,
    AccountBalance,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditGrantKind {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Included => "INCLUDED",
            Self::Prepaid => "PREPAID",
            Self::Bonus => "BONUS",
            Self::AccountBalance => "ACCOUNT_BALANCE",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditGrantKind {
    fn from(value: &str) -> Self {
        match value {
            "INCLUDED" => Self::Included,
            "PREPAID" => Self::Prepaid,
            "BONUS" => Self::Bonus,
            "ACCOUNT_BALANCE" => Self::AccountBalance,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditGrantKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditGrantKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditGrantKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditGrantKind {
    fn encode(&self) -> String {
        self.to_string()
    }
}
