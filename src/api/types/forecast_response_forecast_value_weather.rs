pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForecastResponseForecastValueWeather {
    /// WMO weather interpretation code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

impl ForecastResponseForecastValueWeather {
    pub fn builder() -> ForecastResponseForecastValueWeatherBuilder {
        <ForecastResponseForecastValueWeatherBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForecastResponseForecastValueWeatherBuilder {
    code: Option<i64>,
}

impl ForecastResponseForecastValueWeatherBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForecastResponseForecastValueWeather`].
    pub fn build(self) -> Result<ForecastResponseForecastValueWeather, BuildError> {
        Ok(ForecastResponseForecastValueWeather { code: self.code })
    }
}
