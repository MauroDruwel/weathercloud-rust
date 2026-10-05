pub use crate::prelude::*;

/// Stations as compact arrays. Also contains `owner` and `favorites` arrays (empty unless logged in).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MapDevicesResponse {
    /// Each entry is an array:
    /// `[id, city, lat, lon, status, 0, temp×10, hum, bar×10, wspd×10, wdir, rainrate×10, rain×10, solarrad×10, uvi×10]`
    ///
    /// - `status`: 1=online, 2=recently online, 3=offline
    /// - Divide index 6 and above by 10 to get real units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Vec<serde_json::Value>>>,
    /// Owner's own stations (empty if not logged in)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<Vec<serde_json::Value>>>,
    /// Favourite stations (empty if not logged in)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorites: Option<Vec<Vec<serde_json::Value>>>,
}

impl MapDevicesResponse {
    pub fn builder() -> MapDevicesResponseBuilder {
        <MapDevicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MapDevicesResponseBuilder {
    devices: Option<Vec<Vec<serde_json::Value>>>,
    owner: Option<Vec<Vec<serde_json::Value>>>,
    favorites: Option<Vec<Vec<serde_json::Value>>>,
}

impl MapDevicesResponseBuilder {
    pub fn devices(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.devices = Some(value);
        self
    }

    pub fn owner(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn favorites(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.favorites = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MapDevicesResponse`].
    pub fn build(self) -> Result<MapDevicesResponse, BuildError> {
        Ok(MapDevicesResponse {
            devices: self.devices,
            owner: self.owner,
            favorites: self.favorites,
        })
    }
}
