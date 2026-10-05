pub use crate::prelude::*;

/// Scaled integer sensor readings — divide by 10 for most values.
/// e.g. `temp: 281` → 28.1°C, `bar: 10247` → 1024.7 hPa, `uvi: 70` → UV 7.0
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageDeviceValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dew: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wspd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wspdavg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wspdhi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rainrate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solarrad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uvi: Option<String>,
    /// Indoor temperature (°C, scaled ×10) — if sensor present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempin: Option<String>,
    /// Indoor humidity (%, scaled ×10) — if sensor present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humin: Option<String>,
}

impl PageDeviceValues {
    pub fn builder() -> PageDeviceValuesBuilder {
        <PageDeviceValuesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageDeviceValuesBuilder {
    epoch: Option<String>,
    temp: Option<String>,
    hum: Option<String>,
    dew: Option<String>,
    chill: Option<String>,
    heat: Option<String>,
    bar: Option<String>,
    wdir: Option<String>,
    wspd: Option<String>,
    wspdavg: Option<String>,
    wspdhi: Option<String>,
    rainrate: Option<String>,
    rain: Option<String>,
    solarrad: Option<String>,
    uvi: Option<String>,
    tempin: Option<String>,
    humin: Option<String>,
}

impl PageDeviceValuesBuilder {
    pub fn epoch(mut self, value: impl Into<String>) -> Self {
        self.epoch = Some(value.into());
        self
    }

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

    pub fn chill(mut self, value: impl Into<String>) -> Self {
        self.chill = Some(value.into());
        self
    }

    pub fn heat(mut self, value: impl Into<String>) -> Self {
        self.heat = Some(value.into());
        self
    }

    pub fn bar(mut self, value: impl Into<String>) -> Self {
        self.bar = Some(value.into());
        self
    }

    pub fn wdir(mut self, value: impl Into<String>) -> Self {
        self.wdir = Some(value.into());
        self
    }

    pub fn wspd(mut self, value: impl Into<String>) -> Self {
        self.wspd = Some(value.into());
        self
    }

    pub fn wspdavg(mut self, value: impl Into<String>) -> Self {
        self.wspdavg = Some(value.into());
        self
    }

    pub fn wspdhi(mut self, value: impl Into<String>) -> Self {
        self.wspdhi = Some(value.into());
        self
    }

    pub fn rainrate(mut self, value: impl Into<String>) -> Self {
        self.rainrate = Some(value.into());
        self
    }

    pub fn rain(mut self, value: impl Into<String>) -> Self {
        self.rain = Some(value.into());
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

    pub fn tempin(mut self, value: impl Into<String>) -> Self {
        self.tempin = Some(value.into());
        self
    }

    pub fn humin(mut self, value: impl Into<String>) -> Self {
        self.humin = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PageDeviceValues`].
    pub fn build(self) -> Result<PageDeviceValues, BuildError> {
        Ok(PageDeviceValues {
            epoch: self.epoch,
            temp: self.temp,
            hum: self.hum,
            dew: self.dew,
            chill: self.chill,
            heat: self.heat,
            bar: self.bar,
            wdir: self.wdir,
            wspd: self.wspd,
            wspdavg: self.wspdavg,
            wspdhi: self.wspdhi,
            rainrate: self.rainrate,
            rain: self.rain,
            solarrad: self.solarrad,
            uvi: self.uvi,
            tempin: self.tempin,
            humin: self.humin,
        })
    }
}
