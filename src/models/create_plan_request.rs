// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_config::BillingConfig, entitlement_spec_request::EntitlementSpecRequest,
    plan_add_on_input::PlanAddOnInput, plan_status_enum::PlanStatusEnum,
    plan_type_enum::PlanTypeEnum, price_component_input::PriceComponentInput,
    product_family_id::ProductFamilyId, trial_config::TrialConfig,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatePlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<PlanAddOnInput>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingConfig>,

    pub components: Vec<PriceComponentInput>,

    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Entitlements to attach to this plan's version. Replacing a published plan creates a
    /// new version, and entitlements belong to a version, so passing them here keeps them
    /// attached to whichever version the call produces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<EntitlementSpecRequest>>,

    pub name: String,

    pub plan_type: PlanTypeEnum,

    pub product_family_id: ProductFamilyId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service_rank: Option<i32>,

    pub status: PlanStatusEnum,

    /// The plan's amounts are quoted tax-included ("9.99 incl. VAT"): tax is carved out of
    /// them at invoice time instead of being added on top, so the customer pays the quoted
    /// price whatever rate applies. A customer who bears no tax (reverse charge, exempt,
    /// export) still pays it in full. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<TrialConfig>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreatePlanRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        components: Vec<PriceComponentInput>,
        currency: impl Into<String>,
        name: impl Into<String>,
        plan_type: PlanTypeEnum,
        product_family_id: impl Into<ProductFamilyId>,
        status: PlanStatusEnum,
    ) -> Self {
        Self {
            add_ons: None,
            billing: None,
            components,
            currency: currency.into(),
            description: None,
            entitlements: None,
            name: name.into(),
            plan_type,
            product_family_id: product_family_id.into(),
            self_service_rank: None,
            status,
            tax_inclusive: None,
            trial: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `add_ons`.
    #[must_use]
    pub fn add_ons(mut self, add_ons: impl Into<Vec<PlanAddOnInput>>) -> Self {
        self.add_ons = Some(add_ons.into());
        self
    }

    /// Sets `billing`.
    #[must_use]
    pub fn billing(mut self, billing: impl Into<BillingConfig>) -> Self {
        self.billing = Some(billing.into());
        self
    }

    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets `entitlements`.
    #[must_use]
    pub fn entitlements(mut self, entitlements: impl Into<Vec<EntitlementSpecRequest>>) -> Self {
        self.entitlements = Some(entitlements.into());
        self
    }

    /// Sets `self_service_rank`.
    #[must_use]
    pub fn self_service_rank(mut self, self_service_rank: impl Into<i32>) -> Self {
        self.self_service_rank = Some(self_service_rank.into());
        self
    }

    /// Sets `tax_inclusive`.
    #[must_use]
    pub fn tax_inclusive(mut self, tax_inclusive: impl Into<bool>) -> Self {
        self.tax_inclusive = Some(tax_inclusive.into());
        self
    }

    /// Sets `trial`.
    #[must_use]
    pub fn trial(mut self, trial: impl Into<TrialConfig>) -> Self {
        self.trial = Some(trial.into());
        self
    }
}
