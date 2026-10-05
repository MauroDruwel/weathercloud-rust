pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetEvolutionDeviceHistoryRequest {
    #[serde(default)]
    pub device: String,
    #[serde(default)]
    pub variable: i64,
    pub period: GetEvolutionDeviceHistoryRequestPeriod,
}

impl GetEvolutionDeviceHistoryRequest {
    pub fn builder() -> GetEvolutionDeviceHistoryRequestBuilder {
        <GetEvolutionDeviceHistoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEvolutionDeviceHistoryRequestBuilder {
    device: Option<String>,
    variable: Option<i64>,
    period: Option<GetEvolutionDeviceHistoryRequestPeriod>,
}

impl GetEvolutionDeviceHistoryRequestBuilder {
    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn variable(mut self, value: i64) -> Self {
        self.variable = Some(value);
        self
    }

    pub fn period(mut self, value: GetEvolutionDeviceHistoryRequestPeriod) -> Self {
        self.period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEvolutionDeviceHistoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](GetEvolutionDeviceHistoryRequestBuilder::device)
    /// - [`variable`](GetEvolutionDeviceHistoryRequestBuilder::variable)
    /// - [`period`](GetEvolutionDeviceHistoryRequestBuilder::period)
    pub fn build(self) -> Result<GetEvolutionDeviceHistoryRequest, BuildError> {
        Ok(GetEvolutionDeviceHistoryRequest {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            variable: self
                .variable
                .ok_or_else(|| BuildError::missing_field("variable"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
