pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUpdateStatusDeviceLiveRequest {
    /// Device ID
    #[serde(default)]
    pub d: String,
}

impl GetUpdateStatusDeviceLiveRequest {
    pub fn builder() -> GetUpdateStatusDeviceLiveRequestBuilder {
        <GetUpdateStatusDeviceLiveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUpdateStatusDeviceLiveRequestBuilder {
    d: Option<String>,
}

impl GetUpdateStatusDeviceLiveRequestBuilder {
    pub fn d(mut self, value: impl Into<String>) -> Self {
        self.d = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetUpdateStatusDeviceLiveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`d`](GetUpdateStatusDeviceLiveRequestBuilder::d)
    pub fn build(self) -> Result<GetUpdateStatusDeviceLiveRequest, BuildError> {
        Ok(GetUpdateStatusDeviceLiveRequest {
            d: self.d.ok_or_else(|| BuildError::missing_field("d"))?,
        })
    }
}
