// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    metric_filter::MetricFilter, metric_segmentation_matrix::MetricSegmentationMatrix,
    unit_conversion::UnitConversion,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMetricRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub description: Option<Option<String>>,

    /// Absent = leave filters untouched; present (even empty) = replace them.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub filters: Option<Option<Vec<MetricFilter>>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub name: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub segmentation_matrix: Option<Option<MetricSegmentationMatrix>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub unit_conversion: Option<Option<UnitConversion>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UpdateMetricRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            filters: None,
            name: None,
            segmentation_matrix: None,
            unit_conversion: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(Some(description.into()));
        self
    }

    /// Sends `description` as `null`, clearing it.
    #[must_use]
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// Sets `filters`.
    #[must_use]
    pub fn filters(mut self, filters: impl Into<Vec<MetricFilter>>) -> Self {
        self.filters = Some(Some(filters.into()));
        self
    }

    /// Sends `filters` as `null`, clearing it.
    #[must_use]
    pub fn clear_filters(mut self) -> Self {
        self.filters = Some(None);
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

    /// Sets `segmentation_matrix`.
    #[must_use]
    pub fn segmentation_matrix(
        mut self,
        segmentation_matrix: impl Into<MetricSegmentationMatrix>,
    ) -> Self {
        self.segmentation_matrix = Some(Some(segmentation_matrix.into()));
        self
    }

    /// Sends `segmentation_matrix` as `null`, clearing it.
    #[must_use]
    pub fn clear_segmentation_matrix(mut self) -> Self {
        self.segmentation_matrix = Some(None);
        self
    }

    /// Sets `unit_conversion`.
    #[must_use]
    pub fn unit_conversion(mut self, unit_conversion: impl Into<UnitConversion>) -> Self {
        self.unit_conversion = Some(Some(unit_conversion.into()));
        self
    }

    /// Sends `unit_conversion` as `null`, clearing it.
    #[must_use]
    pub fn clear_unit_conversion(mut self) -> Self {
        self.unit_conversion = Some(None);
        self
    }
}
