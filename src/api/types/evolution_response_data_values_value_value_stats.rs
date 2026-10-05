pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvolutionResponseDataValuesValueValueStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<i64>,
}

impl EvolutionResponseDataValuesValueValueStats {
    pub fn builder() -> EvolutionResponseDataValuesValueValueStatsBuilder {
        <EvolutionResponseDataValuesValueValueStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvolutionResponseDataValuesValueValueStatsBuilder {
    sum: Option<f64>,
    min: Option<f64>,
    min_time: Option<i64>,
    max: Option<f64>,
    max_time: Option<i64>,
}

impl EvolutionResponseDataValuesValueValueStatsBuilder {
    pub fn sum(mut self, value: f64) -> Self {
        self.sum = Some(value);
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    pub fn min_time(mut self, value: i64) -> Self {
        self.min_time = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn max_time(mut self, value: i64) -> Self {
        self.max_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvolutionResponseDataValuesValueValueStats`].
    pub fn build(self) -> Result<EvolutionResponseDataValuesValueValueStats, BuildError> {
        Ok(EvolutionResponseDataValuesValueValueStats {
            sum: self.sum,
            min: self.min,
            min_time: self.min_time,
            max: self.max,
            max_time: self.max_time,
        })
    }
}
