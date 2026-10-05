pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOwnerProfileDeviceLiveRequest {
    #[serde(default)]
    pub d: String,
}

impl GetOwnerProfileDeviceLiveRequest {
    pub fn builder() -> GetOwnerProfileDeviceLiveRequestBuilder {
        <GetOwnerProfileDeviceLiveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOwnerProfileDeviceLiveRequestBuilder {
    d: Option<String>,
}

impl GetOwnerProfileDeviceLiveRequestBuilder {
    pub fn d(mut self, value: impl Into<String>) -> Self {
        self.d = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOwnerProfileDeviceLiveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`d`](GetOwnerProfileDeviceLiveRequestBuilder::d)
    pub fn build(self) -> Result<GetOwnerProfileDeviceLiveRequest, BuildError> {
        Ok(GetOwnerProfileDeviceLiveRequest {
            d: self.d.ok_or_else(|| BuildError::missing_field("d"))?,
        })
    }
}
