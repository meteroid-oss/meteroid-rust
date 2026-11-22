// this file is @generated
use serde::{Deserialize, Serialize};

use super::invoice_line_usage_breakdown_item::InvoiceLineUsageBreakdownItem;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct InvoiceLineUsageBreakdown {
    /// Largest amount first. Past 99 values, the rest are summed in one last item flagged `others`.
    pub items: Vec<InvoiceLineUsageBreakdownItem>,

    /// The event property the usage is broken down by, e.g. `project_id`.
    pub key: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl InvoiceLineUsageBreakdown {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(items: Vec<InvoiceLineUsageBreakdownItem>, key: impl Into<String>) -> Self {
        Self {
            items,
            key: key.into(),
            extra: serde_json::Map::new(),
        }
    }
}
