pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvolutionResponseDataValuesValueValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<EvolutionResponseDataValuesValueValueStats>,
}

impl EvolutionResponseDataValuesValueValue {
    pub fn builder() -> EvolutionResponseDataValuesValueValueBuilder {
        <EvolutionResponseDataValuesValueValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvolutionResponseDataValuesValueValueBuilder {
    samples: Option<i64>,
    stats: Option<EvolutionResponseDataValuesValueValueStats>,
}

impl EvolutionResponseDataValuesValueValueBuilder {
    pub fn samples(mut self, value: i64) -> Self {
        self.samples = Some(value);
        self
    }

    pub fn stats(mut self, value: EvolutionResponseDataValuesValueValueStats) -> Self {
        self.stats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvolutionResponseDataValuesValueValue`].
    pub fn build(self) -> Result<EvolutionResponseDataValuesValueValue, BuildError> {
        Ok(EvolutionResponseDataValuesValueValue {
            samples: self.samples,
            stats: self.stats,
        })
    }
}
