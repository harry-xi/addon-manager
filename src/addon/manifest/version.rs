use std::{cmp::Ordering, hash::Hash};

use semver::Version;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddonVersion {
    Str(SemVerStr),
    Arr([u64; 3]),
}

impl ToString for AddonVersion {
    fn to_string(&self) -> String {
        Version::from(self).to_string()
    }
}

impl PartialEq for AddonVersion {
    fn eq(&self, other: &Self) -> bool {
        use AddonVersion::{Arr, Str};
        match (self, other) {
            (Arr(s), Arr(o)) => s == o,
            (Str(s), Str(o)) => s.inner == o.inner,
            (Arr(a), Str(s)) | (Str(s), Arr(a)) => Version::new(a[0], a[1], a[2]) == s.inner,
        }
    }
}

impl Eq for AddonVersion {}

impl From<&AddonVersion> for Version {
    fn from(value: &AddonVersion) -> Self {
        match value {
            AddonVersion::Str(i) => i.inner.clone(),
            AddonVersion::Arr(i) => Version::new(i[0], i[1], i[2]),
        }
    }
}

impl Ord for AddonVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            Ordering::Equal
        } else {
            Version::from(self).cmp(&other.into())
        }
    }
}
impl PartialOrd for AddonVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for AddonVersion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct SemVerStr {
    inner: Version,
}

struct SemVerVisitor;

impl<'de> Visitor<'de> for SemVerVisitor {
    type Value = SemVerStr;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("must be a semver version string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Version::parse(v) {
            Err(err) => Err(E::custom(err.to_string())),
            Ok(v) => Ok(SemVerStr { inner: v }),
        }
    }
}

impl<'de> Deserialize<'de> for SemVerStr {
    fn deserialize<D>(deserializer: D) -> Result<SemVerStr, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(SemVerVisitor)
    }
}
impl Serialize for SemVerStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.inner.to_string())
    }
}
