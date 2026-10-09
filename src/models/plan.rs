// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    available_parameters::AvailableParameters, currency::Currency, entitlement::Entitlement,
    minimum_commitment::MinimumCommitment, plan_id::PlanId, plan_status_enum::PlanStatusEnum,
    plan_type_enum::PlanTypeEnum, plan_version_id::PlanVersionId, price_component::PriceComponent,
    product_family::ProductFamily, trial_config::TrialConfig,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Plan {
    pub available_parameters: AvailableParameters,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i32>,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,

    pub id: PlanId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_commitment: Option<MinimumCommitment>,

    pub name: String,

    pub net_terms: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start_day: Option<i32>,

    pub plan_type: PlanTypeEnum,

    pub price_components: Vec<PriceComponent>,

    pub product_family: ProductFamily,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service_rank: Option<i32>,

    pub status: PlanStatusEnum,

    /// The plan's amounts are quoted tax-included ("9.99 incl. VAT"): tax is carved out of
    /// them at invoice time instead of being added on top, so the customer pays the quoted
    /// price whatever rate applies. A customer who bears no tax (reverse charge, exempt,
    /// export) still pays it in full. Defaults to `false`.
    pub tax_inclusive: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<TrialConfig>,

    pub version: i32,

    pub version_id: PlanVersionId,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Plan {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        available_parameters: AvailableParameters,
        created_at: chrono::DateTime<chrono::Utc>,
        currency: Currency,
        id: impl Into<PlanId>,
        name: impl Into<String>,
        net_terms: i32,
        plan_type: PlanTypeEnum,
        price_components: Vec<PriceComponent>,
        product_family: ProductFamily,
        status: PlanStatusEnum,
        tax_inclusive: bool,
        version: i32,
        version_id: impl Into<PlanVersionId>,
    ) -> Self {
        Self {
            available_parameters,
            billing_cycles: None,
            created_at,
            currency,
            description: None,
            entitlements: None,
            id: id.into(),
            minimum_commitment: None,
            name: name.into(),
            net_terms,
            period_start_day: None,
            plan_type,
            price_components,
            product_family,
            self_service_rank: None,
            status,
            tax_inclusive,
            trial: None,
            version,
            version_id: version_id.into(),
            extra: serde_json::Map::new(),
        }
    }
}
