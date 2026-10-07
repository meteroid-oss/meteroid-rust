// this file is @generated
use serde::{Deserialize, Serialize};

use super::country_code::CountryCode;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Address {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            state: None,
            zip_code: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `city`.
    #[must_use]
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    /// Sets `country`.
    #[must_use]
    pub fn country(mut self, country: impl Into<CountryCode>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets `line1`.
    #[must_use]
    pub fn line1(mut self, line1: impl Into<String>) -> Self {
        self.line1 = Some(line1.into());
        self
    }

    /// Sets `line2`.
    #[must_use]
    pub fn line2(mut self, line2: impl Into<String>) -> Self {
        self.line2 = Some(line2.into());
        self
    }

    /// Sets `state`.
    #[must_use]
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Sets `zip_code`.
    #[must_use]
    pub fn zip_code(mut self, zip_code: impl Into<String>) -> Self {
        self.zip_code = Some(zip_code.into());
        self
    }
}
