// this file is @generated
use serde::{Deserialize, Serialize};

use super::payment_methods_config::PaymentMethodsConfig;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionUpdateRequest {
    /// If false, invoices will stay in Draft until manually reviewed and finalized.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub auto_advance_invoices: Option<Option<bool>>,

    /// Automatically try to charge the customer's configured payment method on finalize.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub charge_automatically: Option<Option<bool>>,

    /// Partial update of custom property values (merge; send a key with `null` to remove it).
    /// Validated against the tenant's `SUBSCRIPTION` property definitions. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    /// Default memo for invoices
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub invoice_memo: Option<Option<String>>,

    /// Payment terms in days (0 = due on issue)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub net_terms: Option<Option<i32>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub payment_methods_config: Option<Option<PaymentMethodsConfig>>,

    /// Purchase order number
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub purchase_order: Option<Option<String>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl SubscriptionUpdateRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            auto_advance_invoices: None,
            charge_automatically: None,
            custom_properties: None,
            invoice_memo: None,
            net_terms: None,
            payment_methods_config: None,
            purchase_order: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `auto_advance_invoices`.
    #[must_use]
    pub fn auto_advance_invoices(mut self, auto_advance_invoices: impl Into<bool>) -> Self {
        self.auto_advance_invoices = Some(Some(auto_advance_invoices.into()));
        self
    }

    /// Sends `auto_advance_invoices` as `null`, clearing it.
    #[must_use]
    pub fn clear_auto_advance_invoices(mut self) -> Self {
        self.auto_advance_invoices = Some(None);
        self
    }

    /// Sets `charge_automatically`.
    #[must_use]
    pub fn charge_automatically(mut self, charge_automatically: impl Into<bool>) -> Self {
        self.charge_automatically = Some(Some(charge_automatically.into()));
        self
    }

    /// Sends `charge_automatically` as `null`, clearing it.
    #[must_use]
    pub fn clear_charge_automatically(mut self) -> Self {
        self.charge_automatically = Some(None);
        self
    }

    /// Sets `custom_properties`.
    #[must_use]
    pub fn custom_properties(mut self, custom_properties: impl Into<serde_json::Value>) -> Self {
        self.custom_properties = Some(custom_properties.into());
        self
    }

    /// Sets `invoice_memo`.
    #[must_use]
    pub fn invoice_memo(mut self, invoice_memo: impl Into<String>) -> Self {
        self.invoice_memo = Some(Some(invoice_memo.into()));
        self
    }

    /// Sends `invoice_memo` as `null`, clearing it.
    #[must_use]
    pub fn clear_invoice_memo(mut self) -> Self {
        self.invoice_memo = Some(None);
        self
    }

    /// Sets `net_terms`.
    #[must_use]
    pub fn net_terms(mut self, net_terms: impl Into<i32>) -> Self {
        self.net_terms = Some(Some(net_terms.into()));
        self
    }

    /// Sends `net_terms` as `null`, clearing it.
    #[must_use]
    pub fn clear_net_terms(mut self) -> Self {
        self.net_terms = Some(None);
        self
    }

    /// Sets `payment_methods_config`.
    #[must_use]
    pub fn payment_methods_config(
        mut self,
        payment_methods_config: impl Into<PaymentMethodsConfig>,
    ) -> Self {
        self.payment_methods_config = Some(Some(payment_methods_config.into()));
        self
    }

    /// Sends `payment_methods_config` as `null`, clearing it.
    #[must_use]
    pub fn clear_payment_methods_config(mut self) -> Self {
        self.payment_methods_config = Some(None);
        self
    }

    /// Sets `purchase_order`.
    #[must_use]
    pub fn purchase_order(mut self, purchase_order: impl Into<String>) -> Self {
        self.purchase_order = Some(Some(purchase_order.into()));
        self
    }

    /// Sends `purchase_order` as `null`, clearing it.
    #[must_use]
    pub fn clear_purchase_order(mut self) -> Self {
        self.purchase_order = Some(None);
        self
    }
}
