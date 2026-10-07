// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    address::Address, currency::Currency, custom_tax_rate::CustomTaxRate,
    customer_type::CustomerType, invoicing_entity_id::InvoicingEntityId,
    shipping_address::ShippingAddress,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerPatchRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub alias: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub billing_address: Option<Option<Address>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub billing_email: Option<Option<String>>,

    /// BT-10 — the reference the buyer routes invoices by (a Leitweg-ID for German
    /// public bodies). Required by XRechnung.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub buyer_reference: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub currency: Option<Option<Currency>>,

    /// Partial update of custom property values (merge; send a key with `null` to remove it).
    /// Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub custom_taxes: Option<Option<Vec<CustomTaxRate>>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub customer_type: Option<Option<CustomerType>>,

    /// Free-text legal exemption mention surfaced on exempt invoices.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub exemption_reason: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub first_name: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub invoicing_emails: Option<Option<Vec<String>>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub invoicing_entity_id: Option<Option<InvoicingEntityId>>,

    /// Deprecated: use `preferred_locales`. Applied only when `preferred_locales` is absent.
    #[deprecated]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub invoicing_language: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub is_tax_exempt: Option<Option<bool>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub last_name: Option<Option<String>>,

    /// BT-47 — the buyer's national register identifier (SIREN/SIRET, HRB).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub legal_number: Option<Option<String>>,

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
    pub phone: Option<Option<String>>,

    /// Preferred document languages, most-preferred first (BCP-47 tags, e.g.
    /// `["fr-FR", "en"]`); overrides the invoicing entity default. Omit to leave
    /// unchanged, send `[]` to reset to that default.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub preferred_locales: Option<Option<Vec<String>>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub shipping_address: Option<Option<ShippingAddress>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub vat_number: Option<Option<String>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CustomerPatchRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self {
            alias: None,
            billing_address: None,
            billing_email: None,
            buyer_reference: None,
            currency: None,
            custom_properties: None,
            custom_taxes: None,
            customer_type: None,
            exemption_reason: None,
            first_name: None,
            invoicing_emails: None,
            invoicing_entity_id: None,
            invoicing_language: None,
            is_tax_exempt: None,
            last_name: None,
            legal_number: None,
            name: None,
            phone: None,
            preferred_locales: None,
            shipping_address: None,
            vat_number: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `alias`.
    #[must_use]
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(Some(alias.into()));
        self
    }

    /// Sends `alias` as `null`, clearing it.
    #[must_use]
    pub fn clear_alias(mut self) -> Self {
        self.alias = Some(None);
        self
    }

    /// Sets `billing_address`.
    #[must_use]
    pub fn billing_address(mut self, billing_address: impl Into<Address>) -> Self {
        self.billing_address = Some(Some(billing_address.into()));
        self
    }

    /// Sends `billing_address` as `null`, clearing it.
    #[must_use]
    pub fn clear_billing_address(mut self) -> Self {
        self.billing_address = Some(None);
        self
    }

    /// Sets `billing_email`.
    #[must_use]
    pub fn billing_email(mut self, billing_email: impl Into<String>) -> Self {
        self.billing_email = Some(Some(billing_email.into()));
        self
    }

    /// Sends `billing_email` as `null`, clearing it.
    #[must_use]
    pub fn clear_billing_email(mut self) -> Self {
        self.billing_email = Some(None);
        self
    }

    /// Sets `buyer_reference`.
    #[must_use]
    pub fn buyer_reference(mut self, buyer_reference: impl Into<String>) -> Self {
        self.buyer_reference = Some(Some(buyer_reference.into()));
        self
    }

    /// Sends `buyer_reference` as `null`, clearing it.
    #[must_use]
    pub fn clear_buyer_reference(mut self) -> Self {
        self.buyer_reference = Some(None);
        self
    }

    /// Sets `currency`.
    #[must_use]
    pub fn currency(mut self, currency: impl Into<Currency>) -> Self {
        self.currency = Some(Some(currency.into()));
        self
    }

    /// Sends `currency` as `null`, clearing it.
    #[must_use]
    pub fn clear_currency(mut self) -> Self {
        self.currency = Some(None);
        self
    }

    /// Sets `custom_properties`.
    #[must_use]
    pub fn custom_properties(mut self, custom_properties: impl Into<serde_json::Value>) -> Self {
        self.custom_properties = Some(custom_properties.into());
        self
    }

    /// Sets `custom_taxes`.
    #[must_use]
    pub fn custom_taxes(mut self, custom_taxes: impl Into<Vec<CustomTaxRate>>) -> Self {
        self.custom_taxes = Some(Some(custom_taxes.into()));
        self
    }

    /// Sends `custom_taxes` as `null`, clearing it.
    #[must_use]
    pub fn clear_custom_taxes(mut self) -> Self {
        self.custom_taxes = Some(None);
        self
    }

    /// Sets `customer_type`.
    #[must_use]
    pub fn customer_type(mut self, customer_type: impl Into<CustomerType>) -> Self {
        self.customer_type = Some(Some(customer_type.into()));
        self
    }

    /// Sends `customer_type` as `null`, clearing it.
    #[must_use]
    pub fn clear_customer_type(mut self) -> Self {
        self.customer_type = Some(None);
        self
    }

    /// Sets `exemption_reason`.
    #[must_use]
    pub fn exemption_reason(mut self, exemption_reason: impl Into<String>) -> Self {
        self.exemption_reason = Some(Some(exemption_reason.into()));
        self
    }

    /// Sends `exemption_reason` as `null`, clearing it.
    #[must_use]
    pub fn clear_exemption_reason(mut self) -> Self {
        self.exemption_reason = Some(None);
        self
    }

    /// Sets `first_name`.
    #[must_use]
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(Some(first_name.into()));
        self
    }

    /// Sends `first_name` as `null`, clearing it.
    #[must_use]
    pub fn clear_first_name(mut self) -> Self {
        self.first_name = Some(None);
        self
    }

    /// Sets `invoicing_emails`.
    #[must_use]
    pub fn invoicing_emails(mut self, invoicing_emails: impl Into<Vec<String>>) -> Self {
        self.invoicing_emails = Some(Some(invoicing_emails.into()));
        self
    }

    /// Sends `invoicing_emails` as `null`, clearing it.
    #[must_use]
    pub fn clear_invoicing_emails(mut self) -> Self {
        self.invoicing_emails = Some(None);
        self
    }

    /// Sets `invoicing_entity_id`.
    #[must_use]
    pub fn invoicing_entity_id(
        mut self,
        invoicing_entity_id: impl Into<InvoicingEntityId>,
    ) -> Self {
        self.invoicing_entity_id = Some(Some(invoicing_entity_id.into()));
        self
    }

    /// Sends `invoicing_entity_id` as `null`, clearing it.
    #[must_use]
    pub fn clear_invoicing_entity_id(mut self) -> Self {
        self.invoicing_entity_id = Some(None);
        self
    }

    /// Sets `invoicing_language`.
    #[must_use]
    #[deprecated]
    #[allow(deprecated)]
    pub fn invoicing_language(mut self, invoicing_language: impl Into<String>) -> Self {
        self.invoicing_language = Some(Some(invoicing_language.into()));
        self
    }

    /// Sends `invoicing_language` as `null`, clearing it.
    #[must_use]
    #[deprecated]
    #[allow(deprecated)]
    pub fn clear_invoicing_language(mut self) -> Self {
        self.invoicing_language = Some(None);
        self
    }

    /// Sets `is_tax_exempt`.
    #[must_use]
    pub fn is_tax_exempt(mut self, is_tax_exempt: impl Into<bool>) -> Self {
        self.is_tax_exempt = Some(Some(is_tax_exempt.into()));
        self
    }

    /// Sends `is_tax_exempt` as `null`, clearing it.
    #[must_use]
    pub fn clear_is_tax_exempt(mut self) -> Self {
        self.is_tax_exempt = Some(None);
        self
    }

    /// Sets `last_name`.
    #[must_use]
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(Some(last_name.into()));
        self
    }

    /// Sends `last_name` as `null`, clearing it.
    #[must_use]
    pub fn clear_last_name(mut self) -> Self {
        self.last_name = Some(None);
        self
    }

    /// Sets `legal_number`.
    #[must_use]
    pub fn legal_number(mut self, legal_number: impl Into<String>) -> Self {
        self.legal_number = Some(Some(legal_number.into()));
        self
    }

    /// Sends `legal_number` as `null`, clearing it.
    #[must_use]
    pub fn clear_legal_number(mut self) -> Self {
        self.legal_number = Some(None);
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

    /// Sets `phone`.
    #[must_use]
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(Some(phone.into()));
        self
    }

    /// Sends `phone` as `null`, clearing it.
    #[must_use]
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Sets `preferred_locales`.
    #[must_use]
    pub fn preferred_locales(mut self, preferred_locales: impl Into<Vec<String>>) -> Self {
        self.preferred_locales = Some(Some(preferred_locales.into()));
        self
    }

    /// Sends `preferred_locales` as `null`, clearing it.
    #[must_use]
    pub fn clear_preferred_locales(mut self) -> Self {
        self.preferred_locales = Some(None);
        self
    }

    /// Sets `shipping_address`.
    #[must_use]
    pub fn shipping_address(mut self, shipping_address: impl Into<ShippingAddress>) -> Self {
        self.shipping_address = Some(Some(shipping_address.into()));
        self
    }

    /// Sends `shipping_address` as `null`, clearing it.
    #[must_use]
    pub fn clear_shipping_address(mut self) -> Self {
        self.shipping_address = Some(None);
        self
    }

    /// Sets `vat_number`.
    #[must_use]
    pub fn vat_number(mut self, vat_number: impl Into<String>) -> Self {
        self.vat_number = Some(Some(vat_number.into()));
        self
    }

    /// Sends `vat_number` as `null`, clearing it.
    #[must_use]
    pub fn clear_vat_number(mut self) -> Self {
        self.vat_number = Some(None);
        self
    }
}
