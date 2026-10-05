pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForecastResponseForecastValueTemperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
}

impl ForecastResponseForecastValueTemperature {
    pub fn builder() -> ForecastResponseForecastValueTemperatureBuilder {
        <ForecastResponseForecastValueTemperatureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForecastResponseForecastValueTemperatureBuilder {
    max: Option<i64>,
    min: Option<i64>,
}

impl ForecastResponseForecastValueTemperatureBuilder {
    pub fn max(mut self, value: i64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: i64) -> Self {
        self.min = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForecastResponseForecastValueTemperature`].
    pub fn build(self) -> Result<ForecastResponseForecastValueTemperature, BuildError> {
        Ok(ForecastResponseForecastValueTemperature {
            max: self.max,
            min: self.min,
        })
    }
}
