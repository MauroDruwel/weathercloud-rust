pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForecastResponseInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

impl ForecastResponseInfo {
    pub fn builder() -> ForecastResponseInfoBuilder {
        <ForecastResponseInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForecastResponseInfoBuilder {
    source: Option<String>,
    web: Option<String>,
    days: Option<i64>,
}

impl ForecastResponseInfoBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn web(mut self, value: impl Into<String>) -> Self {
        self.web = Some(value.into());
        self
    }

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForecastResponseInfo`].
    pub fn build(self) -> Result<ForecastResponseInfo, BuildError> {
        Ok(ForecastResponseInfo {
            source: self.source,
            web: self.web,
            days: self.days,
        })
    }
}
