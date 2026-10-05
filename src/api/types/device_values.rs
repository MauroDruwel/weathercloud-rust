pub use crate::prelude::*;

/// Live sensor readings
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeviceValues {
    /// Unix timestamp of reading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i64>,
    /// Temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temp: Option<f64>,
    /// Dew point (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew: Option<f64>,
    /// Wind chill (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub chill: Option<f64>,
    /// Heat index (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub heat: Option<f64>,
    /// Relative humidity (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hum: Option<i64>,
    /// Barometric pressure (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bar: Option<f64>,
    /// Instantaneous wind direction (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wdir: Option<i64>,
    /// Average wind direction (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wdiravg: Option<i64>,
    /// Instantaneous wind speed (m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wspd: Option<f64>,
    /// Average wind speed (m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wspdavg: Option<f64>,
    /// Wind gust / high speed (m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wspdhi: Option<f64>,
    /// Rain rate (mm/h)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rainrate: Option<f64>,
    /// Total rain (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rain: Option<f64>,
    /// Solar radiation (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub solarrad: Option<f64>,
    /// UV index (standard units, not ×10 — can be fractional, e.g. 0.9)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uvi: Option<f64>,
    /// Inside temperature (°C) — requires authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tempin: Option<f64>,
    /// Inside relative humidity (%) — requires authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humin: Option<i64>,
    /// Inside heat index (°C) — requires authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub heatin: Option<f64>,
}

impl DeviceValues {
    pub fn builder() -> DeviceValuesBuilder {
        <DeviceValuesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceValuesBuilder {
    epoch: Option<i64>,
    temp: Option<f64>,
    dew: Option<f64>,
    chill: Option<f64>,
    heat: Option<f64>,
    hum: Option<i64>,
    bar: Option<f64>,
    wdir: Option<i64>,
    wdiravg: Option<i64>,
    wspd: Option<f64>,
    wspdavg: Option<f64>,
    wspdhi: Option<f64>,
    rainrate: Option<f64>,
    rain: Option<f64>,
    solarrad: Option<f64>,
    uvi: Option<f64>,
    tempin: Option<f64>,
    humin: Option<i64>,
    heatin: Option<f64>,
}

impl DeviceValuesBuilder {
    pub fn epoch(mut self, value: i64) -> Self {
        self.epoch = Some(value);
        self
    }

    pub fn temp(mut self, value: f64) -> Self {
        self.temp = Some(value);
        self
    }

    pub fn dew(mut self, value: f64) -> Self {
        self.dew = Some(value);
        self
    }

    pub fn chill(mut self, value: f64) -> Self {
        self.chill = Some(value);
        self
    }

    pub fn heat(mut self, value: f64) -> Self {
        self.heat = Some(value);
        self
    }

    pub fn hum(mut self, value: i64) -> Self {
        self.hum = Some(value);
        self
    }

    pub fn bar(mut self, value: f64) -> Self {
        self.bar = Some(value);
        self
    }

    pub fn wdir(mut self, value: i64) -> Self {
        self.wdir = Some(value);
        self
    }

    pub fn wdiravg(mut self, value: i64) -> Self {
        self.wdiravg = Some(value);
        self
    }

    pub fn wspd(mut self, value: f64) -> Self {
        self.wspd = Some(value);
        self
    }

    pub fn wspdavg(mut self, value: f64) -> Self {
        self.wspdavg = Some(value);
        self
    }

    pub fn wspdhi(mut self, value: f64) -> Self {
        self.wspdhi = Some(value);
        self
    }

    pub fn rainrate(mut self, value: f64) -> Self {
        self.rainrate = Some(value);
        self
    }

    pub fn rain(mut self, value: f64) -> Self {
        self.rain = Some(value);
        self
    }

    pub fn solarrad(mut self, value: f64) -> Self {
        self.solarrad = Some(value);
        self
    }

    pub fn uvi(mut self, value: f64) -> Self {
        self.uvi = Some(value);
        self
    }

    pub fn tempin(mut self, value: f64) -> Self {
        self.tempin = Some(value);
        self
    }

    pub fn humin(mut self, value: i64) -> Self {
        self.humin = Some(value);
        self
    }

    pub fn heatin(mut self, value: f64) -> Self {
        self.heatin = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceValues`].
    pub fn build(self) -> Result<DeviceValues, BuildError> {
        Ok(DeviceValues {
            epoch: self.epoch,
            temp: self.temp,
            dew: self.dew,
            chill: self.chill,
            heat: self.heat,
            hum: self.hum,
            bar: self.bar,
            wdir: self.wdir,
            wdiravg: self.wdiravg,
            wspd: self.wspd,
            wspdavg: self.wspdavg,
            wspdhi: self.wspdhi,
            rainrate: self.rainrate,
            rain: self.rain,
            solarrad: self.solarrad,
            uvi: self.uvi,
            tempin: self.tempin,
            humin: self.humin,
            heatin: self.heatin,
        })
    }
}
