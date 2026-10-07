// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    create_subscription_add_on::CreateSubscriptionAddOn,
    create_subscription_components::CreateSubscriptionComponents,
    payment_methods_config::PaymentMethodsConfig, plan_id::PlanId,
    subscription_activation_condition_enum::SubscriptionActivationConditionEnum,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionCreateRequest {
    pub activation_condition: SubscriptionActivationConditionEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<CreateSubscriptionAddOn>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance_invoices: Option<bool>,

    /// Historical import mode: when true, invoices finalized for this subscription keep their
    /// billing-period date as the invoice date instead of being stamped with the emission date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdate_invoices: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day_anchor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_automatically: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_codes: Option<Vec<String>>,

    /// User-defined custom property values, keyed by definition `key`. Validated against the
    /// tenant's subscription definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    pub customer_id_or_alias: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_terms: Option<i32>,

    /// Payment methods configuration. If not specified, inherits from the invoicing entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods_config: Option<PaymentMethodsConfig>,

    pub plan_id: PlanId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_components: Option<CreateSubscriptionComponents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    /// Migration mode: when true with a past start_date, skip creating invoices for past cycles.
    /// The subscription will be set to the current billing period with correct cycle_index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_past_invoices: Option<bool>,

    pub start_date: chrono::NaiveDate,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl SubscriptionCreateRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        activation_condition: SubscriptionActivationConditionEnum,
        customer_id_or_alias: impl Into<String>,
        plan_id: impl Into<PlanId>,
        start_date: chrono::NaiveDate,
    ) -> Self {
        Self {
            activation_condition,
            add_ons: None,
            auto_advance_invoices: None,
            backdate_invoices: None,
            billing_day_anchor: None,
            charge_automatically: None,
            coupon_codes: None,
            custom_properties: None,
            customer_id_or_alias: customer_id_or_alias.into(),
            end_date: None,
            invoice_memo: None,
            net_terms: None,
            payment_methods_config: None,
            plan_id: plan_id.into(),
            price_components: None,
            purchase_order: None,
            skip_past_invoices: None,
            start_date,
            trial_days: None,
            version: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `add_ons`.
    #[must_use]
    pub fn add_ons(mut self, add_ons: impl Into<Vec<CreateSubscriptionAddOn>>) -> Self {
        self.add_ons = Some(add_ons.into());
        self
    }

    /// Sets `auto_advance_invoices`.
    #[must_use]
    pub fn auto_advance_invoices(mut self, auto_advance_invoices: impl Into<bool>) -> Self {
        self.auto_advance_invoices = Some(auto_advance_invoices.into());
        self
    }

    /// Sets `backdate_invoices`.
    #[must_use]
    pub fn backdate_invoices(mut self, backdate_invoices: impl Into<bool>) -> Self {
        self.backdate_invoices = Some(backdate_invoices.into());
        self
    }

    /// Sets `billing_day_anchor`.
    #[must_use]
    pub fn billing_day_anchor(mut self, billing_day_anchor: impl Into<i32>) -> Self {
        self.billing_day_anchor = Some(billing_day_anchor.into());
        self
    }

    /// Sets `charge_automatically`.
    #[must_use]
    pub fn charge_automatically(mut self, charge_automatically: impl Into<bool>) -> Self {
        self.charge_automatically = Some(charge_automatically.into());
        self
    }

    /// Sets `coupon_codes`.
    #[must_use]
    pub fn coupon_codes(mut self, coupon_codes: impl Into<Vec<String>>) -> Self {
        self.coupon_codes = Some(coupon_codes.into());
        self
    }

    /// Sets `custom_properties`.
    #[must_use]
    pub fn custom_properties(mut self, custom_properties: impl Into<serde_json::Value>) -> Self {
        self.custom_properties = Some(custom_properties.into());
        self
    }

    /// Sets `end_date`.
    #[must_use]
    pub fn end_date(mut self, end_date: impl Into<chrono::NaiveDate>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Sets `invoice_memo`.
    #[must_use]
    pub fn invoice_memo(mut self, invoice_memo: impl Into<String>) -> Self {
        self.invoice_memo = Some(invoice_memo.into());
        self
    }

    /// Sets `net_terms`.
    #[must_use]
    pub fn net_terms(mut self, net_terms: impl Into<i32>) -> Self {
        self.net_terms = Some(net_terms.into());
        self
    }

    /// Sets `payment_methods_config`.
    #[must_use]
    pub fn payment_methods_config(
        mut self,
        payment_methods_config: impl Into<PaymentMethodsConfig>,
    ) -> Self {
        self.payment_methods_config = Some(payment_methods_config.into());
        self
    }

    /// Sets `price_components`.
    #[must_use]
    pub fn price_components(
        mut self,
        price_components: impl Into<CreateSubscriptionComponents>,
    ) -> Self {
        self.price_components = Some(price_components.into());
        self
    }

    /// Sets `purchase_order`.
    #[must_use]
    pub fn purchase_order(mut self, purchase_order: impl Into<String>) -> Self {
        self.purchase_order = Some(purchase_order.into());
        self
    }

    /// Sets `skip_past_invoices`.
    #[must_use]
    pub fn skip_past_invoices(mut self, skip_past_invoices: impl Into<bool>) -> Self {
        self.skip_past_invoices = Some(skip_past_invoices.into());
        self
    }

    /// Sets `trial_days`.
    #[must_use]
    pub fn trial_days(mut self, trial_days: impl Into<i32>) -> Self {
        self.trial_days = Some(trial_days.into());
        self
    }

    /// Sets `version`.
    #[must_use]
    pub fn version(mut self, version: impl Into<i32>) -> Self {
        self.version = Some(version.into());
        self
    }
}
