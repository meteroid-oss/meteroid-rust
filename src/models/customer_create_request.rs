// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    address::Address, currency::Currency, custom_tax_rate::CustomTaxRate,
    customer_type::CustomerType, invoicing_entity_id::InvoicingEntityId,
    shipping_address::ShippingAddress,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CustomerCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,

    /// BT-10 — the reference the buyer routes invoices by (a Leitweg-ID for German
    /// public bodies). Required by XRechnung.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,

    pub currency: Currency,

    /// User-defined custom property values, keyed by definition `key`. Validated against the
    /// tenant's `CUSTOMER` property definitions. Omit to leave unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_taxes: Option<Vec<CustomTaxRate>>,

    /// `INDIVIDUAL` requires `first_name`, `last_name`, and a billing-address country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,

    /// Free-text legal exemption mention surfaced on exempt invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_emails: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_entity_id: Option<InvoicingEntityId>,

    /// Deprecated: use `preferred_locales`. Applied only when `preferred_locales` is absent.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tax_exempt: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// BT-47 — the buyer's national register identifier (SIREN/SIRET, HRB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_number: Option<String>,

    /// Required for `COMPANY`. Ignored for `INDIVIDUAL`: derived from `first_name` + `last_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Preferred document languages, most-preferred first (BCP-47 tags, e.g.
    /// `["fr-FR", "en"]`); overrides the invoicing entity default. The first one the
    /// renderer has a template for wins, so an unsupported entry alongside a supported
    /// one just falls through; a list of only unsupported ones is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CustomerCreateRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(currency: Currency) -> Self {
        #[allow(deprecated)]
        Self {
            alias: None,
            billing_address: None,
            billing_email: None,
            buyer_reference: None,
            connected_account_id: None,
            currency,
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
        self.alias = Some(alias.into());
        self
    }

    /// Sets `billing_address`.
    #[must_use]
    pub fn billing_address(mut self, billing_address: impl Into<Address>) -> Self {
        self.billing_address = Some(billing_address.into());
        self
    }

    /// Sets `billing_email`.
    #[must_use]
    pub fn billing_email(mut self, billing_email: impl Into<String>) -> Self {
        self.billing_email = Some(billing_email.into());
        self
    }

    /// Sets `buyer_reference`.
    #[must_use]
    pub fn buyer_reference(mut self, buyer_reference: impl Into<String>) -> Self {
        self.buyer_reference = Some(buyer_reference.into());
        self
    }

    /// Sets `connected_account_id`.
    #[must_use]
    pub fn connected_account_id(mut self, connected_account_id: impl Into<String>) -> Self {
        self.connected_account_id = Some(connected_account_id.into());
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
        self.custom_taxes = Some(custom_taxes.into());
        self
    }

    /// Sets `customer_type`.
    #[must_use]
    pub fn customer_type(mut self, customer_type: impl Into<CustomerType>) -> Self {
        self.customer_type = Some(customer_type.into());
        self
    }

    /// Sets `exemption_reason`.
    #[must_use]
    pub fn exemption_reason(mut self, exemption_reason: impl Into<String>) -> Self {
        self.exemption_reason = Some(exemption_reason.into());
        self
    }

    /// Sets `first_name`.
    #[must_use]
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    /// Sets `invoicing_emails`.
    #[must_use]
    pub fn invoicing_emails(mut self, invoicing_emails: impl Into<Vec<String>>) -> Self {
        self.invoicing_emails = Some(invoicing_emails.into());
        self
    }

    /// Sets `invoicing_entity_id`.
    #[must_use]
    pub fn invoicing_entity_id(
        mut self,
        invoicing_entity_id: impl Into<InvoicingEntityId>,
    ) -> Self {
        self.invoicing_entity_id = Some(invoicing_entity_id.into());
        self
    }

    /// Sets `invoicing_language`.
    #[must_use]
    #[deprecated]
    #[allow(deprecated)]
    pub fn invoicing_language(mut self, invoicing_language: impl Into<String>) -> Self {
        self.invoicing_language = Some(invoicing_language.into());
        self
    }

    /// Sets `is_tax_exempt`.
    #[must_use]
    pub fn is_tax_exempt(mut self, is_tax_exempt: impl Into<bool>) -> Self {
        self.is_tax_exempt = Some(is_tax_exempt.into());
        self
    }

    /// Sets `last_name`.
    #[must_use]
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Sets `legal_number`.
    #[must_use]
    pub fn legal_number(mut self, legal_number: impl Into<String>) -> Self {
        self.legal_number = Some(legal_number.into());
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets `phone`.
    #[must_use]
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    /// Sets `preferred_locales`.
    #[must_use]
    pub fn preferred_locales(mut self, preferred_locales: impl Into<Vec<String>>) -> Self {
        self.preferred_locales = Some(preferred_locales.into());
        self
    }

    /// Sets `shipping_address`.
    #[must_use]
    pub fn shipping_address(mut self, shipping_address: impl Into<ShippingAddress>) -> Self {
        self.shipping_address = Some(shipping_address.into());
        self
    }

    /// Sets `vat_number`.
    #[must_use]
    pub fn vat_number(mut self, vat_number: impl Into<String>) -> Self {
        self.vat_number = Some(vat_number.into());
        self
    }
}
