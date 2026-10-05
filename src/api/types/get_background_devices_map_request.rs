pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBackgroundDevicesMapRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl GetBackgroundDevicesMapRequest {
    pub fn builder() -> GetBackgroundDevicesMapRequestBuilder {
        <GetBackgroundDevicesMapRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBackgroundDevicesMapRequestBuilder {
    user: Option<String>,
}

impl GetBackgroundDevicesMapRequestBuilder {
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetBackgroundDevicesMapRequest`].
    pub fn build(self) -> Result<GetBackgroundDevicesMapRequest, BuildError> {
        Ok(GetBackgroundDevicesMapRequest { user: self.user })
    }
}
