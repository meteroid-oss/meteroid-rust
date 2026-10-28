// this file is @generated
use serde::{Deserialize, Serialize};

use super::{currency::Currency, top_up_collection::TopUpCollection};

/// Credit sold to the customer, charged or invoiced.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateTopUpRequest {
    /// Credit to buy, positive, applying to ALL charges. In minor units of `currency`, charged
    /// one for one without tax: the usage it pays is taxed on the invoice that bills it. With
    /// `credit_unit`, in minor units of that unit (e.g. 500000 is 5,000.00 credits of a unit with
    /// precision 2), charged at the unit's price per credit in the customer's currency, plus tax
    /// under the unit's product.
    pub amount: i64,

    /// CHARGE (default) charges a saved payment method now; SEND_INVOICE issues an invoice and
    /// grants the credit once it is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<TopUpCollection>,

    /// Buy credits of this credit unit (its code) instead of money; excludes `currency`. They are
    /// charged in the customer's currency at the customer's agreed price for the unit (the
    /// wallet's `unit_price`), else its list price, plus tax, under the unit's product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Absolute expiry of the purchased credit; unset never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    /// CHARGE only; defaults to the customer's default payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,

    /// With `credit_unit`: the negotiated price of one credit for this purchase, in major units of
    /// the customer's currency, as a decimal string (e.g. "0.0008"). Defaults to the wallet's
    /// `unit_price` when set, else the list price; the unit must still have a list price in that
    /// currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateTopUpRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            collection: None,
            credit_unit: None,
            currency: None,
            expires_at: None,
            payment_method_id: None,
            unit_price: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `collection`.
    #[must_use]
    pub fn collection(mut self, collection: impl Into<TopUpCollection>) -> Self {
        self.collection = Some(collection.into());
        self
    }

    /// Sets `credit_unit`.
    #[must_use]
    pub fn credit_unit(mut self, credit_unit: impl Into<String>) -> Self {
        self.credit_unit = Some(credit_unit.into());
        self
    }

    /// Sets `currency`.
    #[must_use]
    pub fn currency(mut self, currency: impl Into<Currency>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, expires_at: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    /// Sets `payment_method_id`.
    #[must_use]
    pub fn payment_method_id(mut self, payment_method_id: impl Into<String>) -> Self {
        self.payment_method_id = Some(payment_method_id.into());
        self
    }

    /// Sets `unit_price`.
    #[must_use]
    pub fn unit_price(mut self, unit_price: impl Into<String>) -> Self {
        self.unit_price = Some(unit_price.into());
        self
    }
}
