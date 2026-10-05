pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observer: Option<DeviceProfileObserver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<DeviceProfileFollowers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceProfileDevice>,
}

impl DeviceProfile {
    pub fn builder() -> DeviceProfileBuilder {
        <DeviceProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceProfileBuilder {
    observer: Option<DeviceProfileObserver>,
    followers: Option<DeviceProfileFollowers>,
    device: Option<DeviceProfileDevice>,
}

impl DeviceProfileBuilder {
    pub fn observer(mut self, value: DeviceProfileObserver) -> Self {
        self.observer = Some(value);
        self
    }

    pub fn followers(mut self, value: DeviceProfileFollowers) -> Self {
        self.followers = Some(value);
        self
    }

    pub fn device(mut self, value: DeviceProfileDevice) -> Self {
        self.device = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceProfile`].
    pub fn build(self) -> Result<DeviceProfile, BuildError> {
        Ok(DeviceProfile {
            observer: self.observer,
            followers: self.followers,
            device: self.device,
        })
    }
}
