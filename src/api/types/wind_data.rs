pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WindData {
    /// Unix timestamp (start of period)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    /// 16 compass directions, each with sum and scale array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<WindDataValuesItem>>,
}

impl WindData {
    pub fn builder() -> WindDataBuilder {
        <WindDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WindDataBuilder {
    date: Option<i64>,
    values: Option<Vec<WindDataValuesItem>>,
}

impl WindDataBuilder {
    pub fn date(mut self, value: i64) -> Self {
        self.date = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<WindDataValuesItem>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WindData`].
    pub fn build(self) -> Result<WindData, BuildError> {
        Ok(WindData {
            date: self.date,
            values: self.values,
        })
    }
}
