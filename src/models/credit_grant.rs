// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_applies_to::CreditAppliesTo, credit_cost::CreditCost,
    credit_grant_kind::CreditGrantKind, credit_grant_source::CreditGrantSource,
    credit_grant_status::CreditGrantStatus, credit_grant_type::CreditGrantType, currency::Currency,
    product_id::ProductId,
};

/// A lot of credit given to or bought by a customer. An invoice applies promotional currency
/// credit before tax and paid credit after it. Within each, it draws the grants that belong to
/// the invoiced subscription first, then grants restricted to products (the fewest products
/// first), then usage-only grants, then the others; then by lowest `priority`, then the soonest
/// to lapse.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditGrant {
    /// Amount granted, in minor units of `currency` or `credit_unit`.
    pub amount: i64,

    /// The products whose charges the credit pays, combined with `applies_to`; empty when it
    /// applies to every product.
    pub applicable_product_ids: Vec<ProductId>,

    /// ALL pays any charge on an invoice; USAGE only usage charges.
    pub applies_to: CreditAppliesTo,

    /// Only on the response that created a PAID currency grant whose `cost` was below its
    /// amount: the PROMOTIONAL "Bonus credit" grant created beside it for the difference, with
    /// the same expiry, scope and priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_grant_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<CreditCost>,

    /// `cost` per one major unit of credit, in major units of the cost currency (e.g. "0.008"),
    /// rounded to 12 decimal places, for display only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_basis: Option<String>,

    /// When the grant was recorded.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The credit note that gave the credit back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<String>,

    /// The credit unit (its code) the amounts are in; absent for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit: Option<String>,

    /// Decimal places of `credit_unit`: amounts are integers of 10^-precision credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_unit_precision: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// When the credit became spendable.
    pub effective_at: chrono::DateTime<chrono::Utc>,

    /// When the subscription the grant belongs to ended, if it ended. Unspent credit lapses then
    /// (status EXPIRED), even before `expires_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Lost to expiry; present only on an expired grant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_amount: Option<i64>,

    /// When unspent credit lapses; absent never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The grant's id (`cgr_…`).
    pub id: String,

    /// The invoice that sold the credit, or whose void or negative total gave it back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    /// The group the grant is shown in: INCLUDED, PREPAID, BONUS or ACCOUNT_BALANCE.
    pub kind: CreditGrantKind,

    /// The group's name as Meteroid shows it: "Included with <plan>", "Purchased credit", "Free
    /// credit" or "Account balance".
    pub kind_label: String,

    /// Lower values are drawn first among grants of the same type that pay the same charges; 100
    /// by default.
    pub priority: i32,

    /// Shown on the credit ledger.
    pub reason: String,

    /// Your own reference, e.g. a contract or support ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Not spent yet, in the same minor units.
    pub remaining: i64,

    /// Where the credit came from.
    pub source: CreditGrantSource,

    /// PENDING until `effective_at`, then ACTIVE, DEPLETED once spent, or EXPIRED.
    pub status: CreditGrantStatus,

    /// The subscription the grant belongs to (`sub_…`): it pays only that subscription's
    /// charges, and lapses when the subscription ends. Null when it pays any of the customer's
    /// subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// PAID or PROMOTIONAL: whether it applies after or before tax.
    pub r#type: CreditGrantType,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditGrant {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        amount: i64,
        applicable_product_ids: Vec<ProductId>,
        applies_to: CreditAppliesTo,
        created_at: chrono::DateTime<chrono::Utc>,
        effective_at: chrono::DateTime<chrono::Utc>,
        id: impl Into<String>,
        kind: CreditGrantKind,
        kind_label: impl Into<String>,
        priority: i32,
        reason: impl Into<String>,
        remaining: i64,
        source: CreditGrantSource,
        status: CreditGrantStatus,
        r#type: CreditGrantType,
    ) -> Self {
        Self {
            amount,
            applicable_product_ids,
            applies_to,
            bonus_grant_id: None,
            cost: None,
            cost_basis: None,
            created_at,
            credit_note_id: None,
            credit_unit: None,
            credit_unit_precision: None,
            currency: None,
            effective_at,
            ended_at: None,
            expired_amount: None,
            expires_at: None,
            id: id.into(),
            invoice_id: None,
            kind,
            kind_label: kind_label.into(),
            priority,
            reason: reason.into(),
            reference: None,
            remaining,
            source,
            status,
            subscription_id: None,
            r#type,
            extra: serde_json::Map::new(),
        }
    }
}
