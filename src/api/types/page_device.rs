pub use crate::prelude::*;

/// Station entry returned by page/coordinates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Device ID (numeric string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Station name as set by the owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    /// Altitude in metres
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isWebcam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_webcam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<i64>,
    #[serde(rename = "isFavorite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    /// Seconds since last update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<i64>,
    /// Distance from query point (km)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Scaled integer sensor readings — divide by 10 for most values.
    /// e.g. `temp: 281` → 28.1°C, `bar: 10247` → 1024.7 hPa, `uvi: 70` → UV 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PageDeviceValues>,
}

impl PageDevice {
    pub fn builder() -> PageDeviceBuilder {
        <PageDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageDeviceBuilder {
    r#type: Option<String>,
    code: Option<String>,
    name: Option<String>,
    city: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    elevation: Option<String>,
    image: Option<String>,
    is_webcam: Option<bool>,
    account: Option<i64>,
    is_favorite: Option<bool>,
    update: Option<i64>,
    data: Option<String>,
    values: Option<PageDeviceValues>,
}

impl PageDeviceBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: impl Into<String>) -> Self {
        self.latitude = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: impl Into<String>) -> Self {
        self.longitude = Some(value.into());
        self
    }

    pub fn elevation(mut self, value: impl Into<String>) -> Self {
        self.elevation = Some(value.into());
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

    pub fn account(mut self, value: i64) -> Self {
        self.account = Some(value);
        self
    }

    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    pub fn update(mut self, value: i64) -> Self {
        self.update = Some(value);
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn values(mut self, value: PageDeviceValues) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageDevice`].
    pub fn build(self) -> Result<PageDevice, BuildError> {
        Ok(PageDevice {
            r#type: self.r#type,
            code: self.code,
            name: self.name,
            city: self.city,
            latitude: self.latitude,
            longitude: self.longitude,
            elevation: self.elevation,
            image: self.image,
            is_webcam: self.is_webcam,
            account: self.account,
            is_favorite: self.is_favorite,
            update: self.update,
            data: self.data,
            values: self.values,
        })
    }
}
