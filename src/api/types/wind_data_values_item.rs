pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WindDataValuesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Vec<i64>>,
}

impl WindDataValuesItem {
    pub fn builder() -> WindDataValuesItemBuilder {
        <WindDataValuesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WindDataValuesItemBuilder {
    sum: Option<f64>,
    scale: Option<Vec<i64>>,
}

impl WindDataValuesItemBuilder {
    pub fn sum(mut self, value: f64) -> Self {
        self.sum = Some(value);
        self
    }

    pub fn scale(mut self, value: Vec<i64>) -> Self {
        self.scale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WindDataValuesItem`].
    pub fn build(self) -> Result<WindDataValuesItem, BuildError> {
        Ok(WindDataValuesItem {
            sum: self.sum,
            scale: self.scale,
        })
    }
}
