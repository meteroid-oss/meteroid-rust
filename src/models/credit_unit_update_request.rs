// this file is @generated
use serde::{Deserialize, Serialize};

/// Partial update: an absent field is left unchanged. `code` and `precision` have no field here,
/// they are immutable. `symbol` set to `null` clears it.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreditUnitUpdateRequest {
    /// Blocks the unit from new prices, packs and grants; existing ones keep working.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub archived: Option<Option<bool>>,

    /// Singular display name, 1 to 64 characters.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub name: Option<Option<String>>,

    /// Plural display name, 1 to 64 characters.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub plural_name: Option<Option<String>>,

    /// The product sales of the unit are booked under; `null` clears it, which stops the unit
    /// from being sold.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub product_id: Option<Option<String>>,

    /// Validated by the store (trimmed, at most 8 characters) rather than here: `validator` has
    /// no built-in support for a double `Option`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub symbol: Option<Option<String>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnitUpdateRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            archived: None,
            name: None,
            plural_name: None,
            product_id: None,
            symbol: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `archived`.
    #[must_use]
    pub fn archived(mut self, archived: impl Into<bool>) -> Self {
        self.archived = Some(Some(archived.into()));
        self
    }

    /// Sends `archived` as `null`, clearing it.
    #[must_use]
    pub fn clear_archived(mut self) -> Self {
        self.archived = Some(None);
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(Some(name.into()));
        self
    }

    /// Sends `name` as `null`, clearing it.
    #[must_use]
    pub fn clear_name(mut self) -> Self {
        self.name = Some(None);
        self
    }

    /// Sets `plural_name`.
    #[must_use]
    pub fn plural_name(mut self, plural_name: impl Into<String>) -> Self {
        self.plural_name = Some(Some(plural_name.into()));
        self
    }

    /// Sends `plural_name` as `null`, clearing it.
    #[must_use]
    pub fn clear_plural_name(mut self) -> Self {
        self.plural_name = Some(None);
        self
    }

    /// Sets `product_id`.
    #[must_use]
    pub fn product_id(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = Some(Some(product_id.into()));
        self
    }

    /// Sends `product_id` as `null`, clearing it.
    #[must_use]
    pub fn clear_product_id(mut self) -> Self {
        self.product_id = Some(None);
        self
    }

    /// Sets `symbol`.
    #[must_use]
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(Some(symbol.into()));
        self
    }

    /// Sends `symbol` as `null`, clearing it.
    #[must_use]
    pub fn clear_symbol(mut self) -> Self {
        self.symbol = Some(None);
        self
    }
}
