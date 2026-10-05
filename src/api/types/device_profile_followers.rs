pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceProfileFollowers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

impl DeviceProfileFollowers {
    pub fn builder() -> DeviceProfileFollowersBuilder {
        <DeviceProfileFollowersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceProfileFollowersBuilder {
    number: Option<String>,
}

impl DeviceProfileFollowersBuilder {
    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeviceProfileFollowers`].
    pub fn build(self) -> Result<DeviceProfileFollowers, BuildError> {
        Ok(DeviceProfileFollowers {
            number: self.number,
        })
    }
}
