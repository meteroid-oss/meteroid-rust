// this file is @generated
use serde::{Deserialize, Serialize};

/// A new credit unit.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreditUnitCreateRequest {
    /// Immutable once created: 4 to 16 characters, A-Z/0-9/_, starting with a letter. Never an
    /// ISO currency code (those are always 3 letters).
    pub code: String,

    /// Singular display name, 1 to 64 characters, e.g. "Credit".
    pub name: String,

    /// Plural display name, 1 to 64 characters, e.g. "Credits".
    pub plural_name: String,

    /// Immutable once created: 0 to 6.
    pub precision: i32,

    /// Short display symbol, at most 8 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditUnitCreateRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        plural_name: impl Into<String>,
        precision: i32,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            plural_name: plural_name.into(),
            precision,
            symbol: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `symbol`.
    #[must_use]
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
}
