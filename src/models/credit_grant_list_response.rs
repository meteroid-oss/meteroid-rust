// this file is @generated
use serde::{Deserialize, Serialize};

use super::{credit_grant::CreditGrant, pagination_response::PaginationResponse};

/// A page of credit grants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CreditGrantListResponse {
    pub data: Vec<CreditGrant>,

    pub pagination_meta: PaginationResponse,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreditGrantListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<CreditGrant>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
            extra: serde_json::Map::new(),
        }
    }
}
