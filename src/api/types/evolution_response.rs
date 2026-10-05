pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvolutionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EvolutionResponseData>,
}

impl EvolutionResponse {
    pub fn builder() -> EvolutionResponseBuilder {
        <EvolutionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvolutionResponseBuilder {
    status: Option<String>,
    data: Option<EvolutionResponseData>,
}

impl EvolutionResponseBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn data(mut self, value: EvolutionResponseData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvolutionResponse`].
    pub fn build(self) -> Result<EvolutionResponse, BuildError> {
        Ok(EvolutionResponse {
            status: self.status,
            data: self.data,
        })
    }
}
