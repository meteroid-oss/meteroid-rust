// this file is @generated
use serde::{Deserialize, Serialize};

use super::unit_conversion_rounding_enum::UnitConversionRoundingEnum;

/// Converts the aggregated usage into the unit prices are set in, e.g. tokens priced per 1M.
/// The usage is divided by `factor`, then rounded. Rounding applies to each priced group's
/// total for the billing period: each group of a segmented or grouped metric, and each matrix
/// cell, is rounded on its own, never each event.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnitConversion {
    /// Raw units per priced unit, e.g. `1000000` to price per 1M.
    pub factor: i32,

    pub rounding: UnitConversionRoundingEnum,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UnitConversion {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(factor: i32, rounding: UnitConversionRoundingEnum) -> Self {
        Self {
            factor,
            rounding,
            extra: serde_json::Map::new(),
        }
    }
}
