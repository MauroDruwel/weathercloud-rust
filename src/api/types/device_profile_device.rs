pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceProfileDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl DeviceProfileDevice {
    pub fn builder() -> DeviceProfileDeviceBuilder {
        <DeviceProfileDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceProfileDeviceBuilder {
    brand: Option<String>,
    model: Option<String>,
}

impl DeviceProfileDeviceBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeviceProfileDevice`].
    pub fn build(self) -> Result<DeviceProfileDevice, BuildError> {
        Ok(DeviceProfileDevice {
            brand: self.brand,
            model: self.model,
        })
    }
}
