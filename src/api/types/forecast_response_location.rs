pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForecastResponseLocation {
    /// Nearest city name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ForecastResponseLocation {
    pub fn builder() -> ForecastResponseLocationBuilder {
        <ForecastResponseLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForecastResponseLocationBuilder {
    name: Option<String>,
}

impl ForecastResponseLocationBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForecastResponseLocation`].
    pub fn build(self) -> Result<ForecastResponseLocation, BuildError> {
        Ok(ForecastResponseLocation { name: self.name })
    }
}
