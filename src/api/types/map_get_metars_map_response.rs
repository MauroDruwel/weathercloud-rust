pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetMetarsMapResponse {
    /// Each entry: `[icao, city, lat, lon, status, 0, wdir, hum, bar*10, wspd*10, wdir2, 0, "", "", ""]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metars: Option<Vec<Vec<serde_json::Value>>>,
}

impl GetMetarsMapResponse {
    pub fn builder() -> GetMetarsMapResponseBuilder {
        <GetMetarsMapResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMetarsMapResponseBuilder {
    metars: Option<Vec<Vec<serde_json::Value>>>,
}

impl GetMetarsMapResponseBuilder {
    pub fn metars(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.metars = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetMetarsMapResponse`].
    pub fn build(self) -> Result<GetMetarsMapResponse, BuildError> {
        Ok(GetMetarsMapResponse {
            metars: self.metars,
        })
    }
}
