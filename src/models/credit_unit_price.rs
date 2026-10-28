// this file is @generated
use serde::{Deserialize, Serialize};

/// What one credit sells for in a currency. A price is never edited: setting a new one for the
/// same currency archives it.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditUnitPrice {
    /// When a newer price, or its removal, replaced it; absent while active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,

    /// When the price was set.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// ISO 4217 code of the currency the unit sells in.
    pub currency: String,

    /// The price's id (`cup_…`).
    pub id: String,

    /// Major units of `currency` per credit, as a decimal string (e.g. "0.001").
    pub unit_price: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnitPrice {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        currency: impl Into<String>,
        id: impl Into<String>,
        unit_price: impl Into<String>,
    ) -> Self {
        Self {
            archived_at: None,
            created_at,
            currency: currency.into(),
            id: id.into(),
            unit_price: unit_price.into(),
            extra: serde_json::Map::new(),
        }
    }
}
