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
pub struct BillableMetricId(String);

impl BillableMetricId {
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

impl std::ops::Deref for BillableMetricId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BillableMetricId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for BillableMetricId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for BillableMetricId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for BillableMetricId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for BillableMetricId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&String> for BillableMetricId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&BillableMetricId> for BillableMetricId {
    fn from(value: &BillableMetricId) -> Self {
        value.clone()
    }
}

impl From<BillableMetricId> for String {
    fn from(value: BillableMetricId) -> Self {
        value.0
    }
}

impl PartialEq<str> for BillableMetricId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for BillableMetricId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for BillableMetricId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl crate::request::QueryParamValue for BillableMetricId {
    fn encode(&self) -> String {
        self.0.clone()
    }
}
