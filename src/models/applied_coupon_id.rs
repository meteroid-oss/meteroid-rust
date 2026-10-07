// this file is @generated

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(transparent)]
pub struct AppliedCouponId(String);

impl AppliedCouponId {
    /// The value, as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The value as a `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::ops::Deref for AppliedCouponId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AppliedCouponId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for AppliedCouponId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AppliedCouponId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for AppliedCouponId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for AppliedCouponId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&String> for AppliedCouponId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&AppliedCouponId> for AppliedCouponId {
    fn from(value: &AppliedCouponId) -> Self {
        value.clone()
    }
}

impl From<AppliedCouponId> for String {
    fn from(value: AppliedCouponId) -> Self {
        value.0
    }
}

impl PartialEq<str> for AppliedCouponId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for AppliedCouponId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for AppliedCouponId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl crate::request::QueryParamValue for AppliedCouponId {
    fn encode(&self) -> String {
        self.0.clone()
    }
}
