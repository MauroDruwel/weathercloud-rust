pub use crate::prelude::*;

/// Query parameters for getWindRose
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWindRoseQueryRequest {
    #[serde(default)]
    pub code: String,
}

impl GetWindRoseQueryRequest {
    pub fn builder() -> GetWindRoseQueryRequestBuilder {
        <GetWindRoseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWindRoseQueryRequestBuilder {
    code: Option<String>,
}

impl GetWindRoseQueryRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWindRoseQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GetWindRoseQueryRequestBuilder::code)
    pub fn build(self) -> Result<GetWindRoseQueryRequest, BuildError> {
        Ok(GetWindRoseQueryRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
