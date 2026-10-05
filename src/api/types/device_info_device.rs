pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceInfoDevice {
    /// Account type (0 = free)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<i64>,
    /// "1" = online, "2" = recently online, "3" = offline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// City name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Altitude in metres (as string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<String>,
    /// URL to station photo, or null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isWebcam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_webcam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social: Option<bool>,
    /// Seconds since last update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<i64>,
}

impl DeviceInfoDevice {
    pub fn builder() -> DeviceInfoDeviceBuilder {
        <DeviceInfoDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceInfoDeviceBuilder {
    account: Option<i64>,
    status: Option<String>,
    city: Option<String>,
    altitude: Option<String>,
    image: Option<String>,
    is_webcam: Option<bool>,
    favorite: Option<bool>,
    social: Option<bool>,
    update: Option<i64>,
}

impl DeviceInfoDeviceBuilder {
    pub fn account(mut self, value: i64) -> Self {
        self.account = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn altitude(mut self, value: impl Into<String>) -> Self {
        self.altitude = Some(value.into());
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn is_webcam(mut self, value: bool) -> Self {
        self.is_webcam = Some(value);
        self
    }

    pub fn favorite(mut self, value: bool) -> Self {
        self.favorite = Some(value);
        self
    }

    pub fn social(mut self, value: bool) -> Self {
        self.social = Some(value);
        self
    }

    pub fn update(mut self, value: i64) -> Self {
        self.update = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceInfoDevice`].
    pub fn build(self) -> Result<DeviceInfoDevice, BuildError> {
        Ok(DeviceInfoDevice {
            account: self.account,
            status: self.status,
            city: self.city,
            altitude: self.altitude,
            image: self.image,
            is_webcam: self.is_webcam,
            favorite: self.favorite,
            social: self.social,
            update: self.update,
        })
    }
}
