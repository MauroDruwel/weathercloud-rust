pub use crate::prelude::*;

/// Query parameters for getStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStatsQueryRequest {
    #[serde(default)]
    pub code: String,
}

impl GetStatsQueryRequest {
    pub fn builder() -> GetStatsQueryRequestBuilder {
        <GetStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStatsQueryRequestBuilder {
    code: Option<String>,
}

impl GetStatsQueryRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GetStatsQueryRequestBuilder::code)
    pub fn build(self) -> Result<GetStatsQueryRequest, BuildError> {
        Ok(GetStatsQueryRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
