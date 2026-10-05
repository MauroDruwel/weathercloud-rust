pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDevicesMapRequest {
    /// Filter by user (empty = all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// lat,lon,zoom format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl GetDevicesMapRequest {
    pub fn builder() -> GetDevicesMapRequestBuilder {
        <GetDevicesMapRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDevicesMapRequestBuilder {
    user: Option<String>,
    location: Option<String>,
}

impl GetDevicesMapRequestBuilder {
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDevicesMapRequest`].
    pub fn build(self) -> Result<GetDevicesMapRequest, BuildError> {
        Ok(GetDevicesMapRequest {
            user: self.user,
            location: self.location,
        })
    }
}
