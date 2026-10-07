// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_metric_aggregate_enum::BillingMetricAggregateEnum, metric_filter::MetricFilter,
    metric_segmentation_matrix::MetricSegmentationMatrix, product_family_id::ProductFamilyId,
    product_id::ProductId, unit_conversion::UnitConversion,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateMetricRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_key: Option<String>,

    pub aggregation_type: BillingMetricAggregateEnum,

    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Pre-aggregation property filters. Optional and backward-compatible; omit for none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MetricFilter>>,

    pub name: String,

    pub product_family_id: ProductFamilyId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_matrix: Option<MetricSegmentationMatrix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_conversion: Option<UnitConversion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_group_key: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateMetricRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        aggregation_type: BillingMetricAggregateEnum,
        code: impl Into<String>,
        name: impl Into<String>,
        product_family_id: impl Into<ProductFamilyId>,
    ) -> Self {
        Self {
            aggregation_key: None,
            aggregation_type,
            code: code.into(),
            description: None,
            filters: None,
            name: name.into(),
            product_family_id: product_family_id.into(),
            product_id: None,
            segmentation_matrix: None,
            unit_conversion: None,
            usage_group_key: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `aggregation_key`.
    #[must_use]
    pub fn aggregation_key(mut self, aggregation_key: impl Into<String>) -> Self {
        self.aggregation_key = Some(aggregation_key.into());
        self
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets `filters`.
    #[must_use]
    pub fn filters(mut self, filters: impl Into<Vec<MetricFilter>>) -> Self {
        self.filters = Some(filters.into());
        self
    }

    /// Sets `product_id`.
    #[must_use]
    pub fn product_id(mut self, product_id: impl Into<ProductId>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }

    /// Sets `segmentation_matrix`.
    #[must_use]
    pub fn segmentation_matrix(
        mut self,
        segmentation_matrix: impl Into<MetricSegmentationMatrix>,
    ) -> Self {
        self.segmentation_matrix = Some(segmentation_matrix.into());
        self
    }

    /// Sets `unit_conversion`.
    #[must_use]
    pub fn unit_conversion(mut self, unit_conversion: impl Into<UnitConversion>) -> Self {
        self.unit_conversion = Some(unit_conversion.into());
        self
    }

    /// Sets `usage_group_key`.
    #[must_use]
    pub fn usage_group_key(mut self, usage_group_key: impl Into<String>) -> Self {
        self.usage_group_key = Some(usage_group_key.into());
        self
    }
}
