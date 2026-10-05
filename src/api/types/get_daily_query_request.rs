pub use crate::prelude::*;

/// Query parameters for getDaily
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDailyQueryRequest {
    #[serde(default)]
    pub id: String,
}

impl GetDailyQueryRequest {
    pub fn builder() -> GetDailyQueryRequestBuilder {
        <GetDailyQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDailyQueryRequestBuilder {
    id: Option<String>,
}

impl GetDailyQueryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDailyQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDailyQueryRequestBuilder::id)
    pub fn build(self) -> Result<GetDailyQueryRequest, BuildError> {
        Ok(GetDailyQueryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
