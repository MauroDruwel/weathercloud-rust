pub use crate::prelude::*;

/// Current sensor readings as strings.
/// Unavailable sensors return `"-3276.8"` (float sensors) or `"-32768"` (integer sensors).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceInfoValues {
    /// Temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp: Option<String>,
    /// Relative humidity (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hum: Option<String>,
    /// Dew point (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dew: Option<String>,
    /// Average wind speed (m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wspdavg: Option<String>,
    /// Average wind direction (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wdiravg: Option<String>,
    /// Barometric pressure (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Total rain (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<String>,
    /// Rain rate (mm/h)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rainrate: Option<String>,
    /// Solar radiation (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solarrad: Option<String>,
    /// UV index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uvi: Option<String>,
}

impl DeviceInfoValues {
    pub fn builder() -> DeviceInfoValuesBuilder {
        <DeviceInfoValuesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceInfoValuesBuilder {
    temp: Option<String>,
    hum: Option<String>,
    dew: Option<String>,
    wspdavg: Option<String>,
    wdiravg: Option<String>,
    bar: Option<String>,
    rain: Option<String>,
    rainrate: Option<String>,
    solarrad: Option<String>,
    uvi: Option<String>,
}

impl DeviceInfoValuesBuilder {
    pub fn temp(mut self, value: impl Into<String>) -> Self {
        self.temp = Some(value.into());
        self
    }

    pub fn hum(mut self, value: impl Into<String>) -> Self {
        self.hum = Some(value.into());
        self
    }

    pub fn dew(mut self, value: impl Into<String>) -> Self {
        self.dew = Some(value.into());
        self
    }

    pub fn wspdavg(mut self, value: impl Into<String>) -> Self {
        self.wspdavg = Some(value.into());
        self
    }

    pub fn wdiravg(mut self, value: impl Into<String>) -> Self {
        self.wdiravg = Some(value.into());
        self
    }

    pub fn bar(mut self, value: impl Into<String>) -> Self {
        self.bar = Some(value.into());
        self
    }

    pub fn rain(mut self, value: impl Into<String>) -> Self {
        self.rain = Some(value.into());
        self
    }

    pub fn rainrate(mut self, value: impl Into<String>) -> Self {
        self.rainrate = Some(value.into());
        self
    }

    pub fn solarrad(mut self, value: impl Into<String>) -> Self {
        self.solarrad = Some(value.into());
        self
    }

    pub fn uvi(mut self, value: impl Into<String>) -> Self {
        self.uvi = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeviceInfoValues`].
    pub fn build(self) -> Result<DeviceInfoValues, BuildError> {
        Ok(DeviceInfoValues {
            temp: self.temp,
            hum: self.hum,
            dew: self.dew,
            wspdavg: self.wspdavg,
            wdiravg: self.wdiravg,
            bar: self.bar,
            rain: self.rain,
            rainrate: self.rainrate,
            solarrad: self.solarrad,
            uvi: self.uvi,
        })
    }
}
