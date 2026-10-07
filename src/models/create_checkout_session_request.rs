// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    coupon_id::CouponId, create_subscription_add_on::CreateSubscriptionAddOn,
    create_subscription_components::CreateSubscriptionComponents,
    payment_methods_config::PaymentMethodsConfig, plan_version_id::PlanVersionId,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateCheckoutSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<CreateSubscriptionAddOn>>,

    /// If false, invoices will stay in Draft until manually reviewed and finalized. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance_invoices: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day_anchor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_start_date: Option<chrono::NaiveDate>,

    /// Absolute http(s) URL offered to the customer to leave the checkout without paying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,

    /// Automatically try to charge the customer's configured payment method on finalize. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_automatically: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<CreateSubscriptionComponents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_ids: Option<Vec<CouponId>>,

    /// Customer ID or alias
    pub customer_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,

    /// Session expiry time in hours. Default is 1 hour for self-serve checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_threshold: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_terms: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods_config: Option<PaymentMethodsConfig>,

    pub plan_version_id: PlanVersionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    /// Absolute http(s) URL the customer is sent to after a successful checkout.
    /// `checkout_session_id` is appended as a query parameter. Without it the customer stays on
    /// the hosted confirmation page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_duration_days: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateCheckoutSessionRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(customer_id: impl Into<String>, plan_version_id: impl Into<PlanVersionId>) -> Self {
        Self {
            add_ons: None,
            auto_advance_invoices: None,
            billing_day_anchor: None,
            billing_start_date: None,
            cancel_url: None,
            charge_automatically: None,
            components: None,
            coupon_code: None,
            coupon_ids: None,
            customer_id: customer_id.into(),
            end_date: None,
            expires_in_hours: None,
            invoice_memo: None,
            invoice_threshold: None,
            metadata: None,
            net_terms: None,
            payment_methods_config: None,
            plan_version_id: plan_version_id.into(),
            purchase_order: None,
            success_url: None,
            trial_duration_days: None,
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

    /// Sets `billing_day_anchor`.
    #[must_use]
    pub fn billing_day_anchor(mut self, billing_day_anchor: impl Into<i32>) -> Self {
        self.billing_day_anchor = Some(billing_day_anchor.into());
        self
    }

    /// Sets `billing_start_date`.
    #[must_use]
    pub fn billing_start_date(mut self, billing_start_date: impl Into<chrono::NaiveDate>) -> Self {
        self.billing_start_date = Some(billing_start_date.into());
        self
    }

    /// Sets `cancel_url`.
    #[must_use]
    pub fn cancel_url(mut self, cancel_url: impl Into<String>) -> Self {
        self.cancel_url = Some(cancel_url.into());
        self
    }

    /// Sets `charge_automatically`.
    #[must_use]
    pub fn charge_automatically(mut self, charge_automatically: impl Into<bool>) -> Self {
        self.charge_automatically = Some(charge_automatically.into());
        self
    }

    /// Sets `components`.
    #[must_use]
    pub fn components(mut self, components: impl Into<CreateSubscriptionComponents>) -> Self {
        self.components = Some(components.into());
        self
    }

    /// Sets `coupon_code`.
    #[must_use]
    pub fn coupon_code(mut self, coupon_code: impl Into<String>) -> Self {
        self.coupon_code = Some(coupon_code.into());
        self
    }

    /// Sets `coupon_ids`.
    #[must_use]
    pub fn coupon_ids(mut self, coupon_ids: impl Into<Vec<CouponId>>) -> Self {
        self.coupon_ids = Some(coupon_ids.into());
        self
    }

    /// Sets `end_date`.
    #[must_use]
    pub fn end_date(mut self, end_date: impl Into<chrono::NaiveDate>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Sets `expires_in_hours`.
    #[must_use]
    pub fn expires_in_hours(mut self, expires_in_hours: impl Into<i32>) -> Self {
        self.expires_in_hours = Some(expires_in_hours.into());
        self
    }

    /// Sets `invoice_memo`.
    #[must_use]
    pub fn invoice_memo(mut self, invoice_memo: impl Into<String>) -> Self {
        self.invoice_memo = Some(invoice_memo.into());
        self
    }

    /// Sets `invoice_threshold`.
    #[must_use]
    pub fn invoice_threshold(
        mut self,
        invoice_threshold: impl Into<rust_decimal::Decimal>,
    ) -> Self {
        self.invoice_threshold = Some(invoice_threshold.into());
        self
    }

    /// Sets `metadata`.
    #[must_use]
    pub fn metadata(mut self, metadata: impl Into<serde_json::Value>) -> Self {
        self.metadata = Some(metadata.into());
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

    /// Sets `purchase_order`.
    #[must_use]
    pub fn purchase_order(mut self, purchase_order: impl Into<String>) -> Self {
        self.purchase_order = Some(purchase_order.into());
        self
    }

    /// Sets `success_url`.
    #[must_use]
    pub fn success_url(mut self, success_url: impl Into<String>) -> Self {
        self.success_url = Some(success_url.into());
        self
    }

    /// Sets `trial_duration_days`.
    #[must_use]
    pub fn trial_duration_days(mut self, trial_duration_days: impl Into<i32>) -> Self {
        self.trial_duration_days = Some(trial_duration_days.into());
        self
    }
}
