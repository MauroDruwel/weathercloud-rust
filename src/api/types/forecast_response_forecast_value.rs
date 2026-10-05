pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForecastResponseForecastValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<ForecastResponseForecastValueWeather>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<ForecastResponseForecastValueTemperature>,
}

impl ForecastResponseForecastValue {
    pub fn builder() -> ForecastResponseForecastValueBuilder {
        <ForecastResponseForecastValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForecastResponseForecastValueBuilder {
    weather: Option<ForecastResponseForecastValueWeather>,
    temperature: Option<ForecastResponseForecastValueTemperature>,
}

impl ForecastResponseForecastValueBuilder {
    pub fn weather(mut self, value: ForecastResponseForecastValueWeather) -> Self {
        self.weather = Some(value);
        self
    }

    pub fn temperature(mut self, value: ForecastResponseForecastValueTemperature) -> Self {
        self.temperature = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForecastResponseForecastValue`].
    pub fn build(self) -> Result<ForecastResponseForecastValue, BuildError> {
        Ok(ForecastResponseForecastValue {
            weather: self.weather,
            temperature: self.temperature,
        })
    }
}
