pub use crate::prelude::*;

/// Current readings and period statistics for all sensors.
/// All values are `[unix_timestamp, value]` tuples (`TimestampValue`).
///
/// **Key pattern:** `{sensor}_{period}_{type}`
/// - Sensors: `temp`, `tempin`, `dew`, `chill`, `heat`, `heatin`, `hum`, `humin`, `bar`, `wdir`, `wdiravg`, `wspd`, `wspdavg`, `wspdhi`, `rainrate`, `rain`, `solarrad`, `uvi`
/// - Period types: `current`, `day_max`, `day_min`, `month_max`, `month_min`, `year_max`, `year_min`
/// - Rain also has: `day_total`, `month_total`, `year_total`
/// - Solarrad also has: `day_hours`, `month_hours`, `year_hours`
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceStats {
    /// Unix timestamp of last reading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<i64>,
}

impl DeviceStats {
    pub fn builder() -> DeviceStatsBuilder {
        <DeviceStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceStatsBuilder {
    last_update: Option<i64>,
}

impl DeviceStatsBuilder {
    pub fn last_update(mut self, value: i64) -> Self {
        self.last_update = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceStats`].
    pub fn build(self) -> Result<DeviceStats, BuildError> {
        Ok(DeviceStats {
            last_update: self.last_update,
        })
    }
}
