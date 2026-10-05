pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvolutionResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Keyed by variable code (e.g. "101")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<HashMap<String, EvolutionResponseDataSummaryValue>>,
    /// Keyed by Unix timestamp (hour buckets), then by variable code.
    /// Each entry has `samples` and `stats`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<HashMap<String, HashMap<String, EvolutionResponseDataValuesValueValue>>>,
}

impl EvolutionResponseData {
    pub fn builder() -> EvolutionResponseDataBuilder {
        <EvolutionResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvolutionResponseDataBuilder {
    timezone: Option<String>,
    summary: Option<HashMap<String, EvolutionResponseDataSummaryValue>>,
    values: Option<HashMap<String, HashMap<String, EvolutionResponseDataValuesValueValue>>>,
}

impl EvolutionResponseDataBuilder {
    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn summary(mut self, value: HashMap<String, EvolutionResponseDataSummaryValue>) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn values(
        mut self,
        value: HashMap<String, HashMap<String, EvolutionResponseDataValuesValueValue>>,
    ) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvolutionResponseData`].
    pub fn build(self) -> Result<EvolutionResponseData, BuildError> {
        Ok(EvolutionResponseData {
            timezone: self.timezone,
            summary: self.summary,
            values: self.values,
        })
    }
}
