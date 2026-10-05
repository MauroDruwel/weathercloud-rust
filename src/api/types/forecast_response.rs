pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForecastResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ForecastResponseInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ForecastResponseLocation>,
    /// Keyed by date string (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<HashMap<String, ForecastResponseForecastValue>>,
}

impl ForecastResponse {
    pub fn builder() -> ForecastResponseBuilder {
        <ForecastResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForecastResponseBuilder {
    info: Option<ForecastResponseInfo>,
    location: Option<ForecastResponseLocation>,
    forecast: Option<HashMap<String, ForecastResponseForecastValue>>,
}

impl ForecastResponseBuilder {
    pub fn info(mut self, value: ForecastResponseInfo) -> Self {
        self.info = Some(value);
        self
    }

    pub fn location(mut self, value: ForecastResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn forecast(mut self, value: HashMap<String, ForecastResponseForecastValue>) -> Self {
        self.forecast = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForecastResponse`].
    pub fn build(self) -> Result<ForecastResponse, BuildError> {
        Ok(ForecastResponse {
            info: self.info,
            location: self.location,
            forecast: self.forecast,
        })
    }
}
