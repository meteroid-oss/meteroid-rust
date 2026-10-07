// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_config::BillingConfig, entitlement_spec_request::EntitlementSpecRequest,
    minimum_commitment_input::MinimumCommitmentInput, plan_add_on_input::PlanAddOnInput,
    plan_status_enum::PlanStatusEnum, price_component_input::PriceComponentInput,
    trial_config::TrialConfig,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReplacePlanRequest {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_commitment: Option<MinimumCommitmentInput>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PlanStatusEnum>,

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

impl ReplacePlanRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        components: Vec<PriceComponentInput>,
        currency: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            add_ons: None,
            billing: None,
            components,
            currency: currency.into(),
            description: None,
            entitlements: None,
            minimum_commitment: None,
            name: name.into(),
            status: None,
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

    /// Sets `minimum_commitment`.
    #[must_use]
    pub fn minimum_commitment(
        mut self,
        minimum_commitment: impl Into<MinimumCommitmentInput>,
    ) -> Self {
        self.minimum_commitment = Some(minimum_commitment.into());
        self
    }

    /// Sets `status`.
    #[must_use]
    pub fn status(mut self, status: impl Into<PlanStatusEnum>) -> Self {
        self.status = Some(status.into());
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
