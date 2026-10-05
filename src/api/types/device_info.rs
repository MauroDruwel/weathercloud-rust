pub use crate::prelude::*;

/// Station metadata and current sensor snapshot.
/// Note: sensor values are returned as strings; unavailable sensors show `-3276.8` / `-32768`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceInfoDevice>,
    /// Current sensor readings as strings.
    /// Unavailable sensors return `"-3276.8"` (float sensors) or `"-32768"` (integer sensors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<DeviceInfoValues>,
}

impl DeviceInfo {
    pub fn builder() -> DeviceInfoBuilder {
        <DeviceInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceInfoBuilder {
    device: Option<DeviceInfoDevice>,
    values: Option<DeviceInfoValues>,
}

impl DeviceInfoBuilder {
    pub fn device(mut self, value: DeviceInfoDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn values(mut self, value: DeviceInfoValues) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceInfo`].
    pub fn build(self) -> Result<DeviceInfo, BuildError> {
        Ok(DeviceInfo {
            device: self.device,
            values: self.values,
        })
    }
}
