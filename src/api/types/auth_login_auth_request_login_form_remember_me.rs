pub use crate::prelude::*;

/// Keep the user logged in
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoginAuthRequestLoginFormRememberMe {
    Zero,
    One,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LoginAuthRequestLoginFormRememberMe {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Zero => serializer.serialize_str("0"),
            Self::One => serializer.serialize_str("1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LoginAuthRequestLoginFormRememberMe {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LoginAuthRequestLoginFormRememberMe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
