//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Auth**
//! - **DeviceLive**
//! - **DeviceHistory**
//! - **Forecast**
//! - **Map**
//! - **Stations**
//! - **Metar**

use crate::{ApiError, ClientConfig};

pub mod auth;
pub mod device_history;
pub mod device_live;
pub mod forecast;
pub mod map;
pub mod metar;
pub mod stations;
pub struct WeathercloudClient {
    pub config: ClientConfig,
    pub auth: AuthClient,
    pub device_live: DeviceLiveClient,
    pub device_history: DeviceHistoryClient,
    pub forecast: ForecastClient,
    pub map: MapClient,
    pub stations: StationsClient,
    pub metar: MetarClient,
}

impl WeathercloudClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            auth: AuthClient::new(config.clone())?,
            device_live: DeviceLiveClient::new(config.clone())?,
            device_history: DeviceHistoryClient::new(config.clone())?,
            forecast: ForecastClient::new(config.clone())?,
            map: MapClient::new(config.clone())?,
            stations: StationsClient::new(config.clone())?,
            metar: MetarClient::new(config.clone())?,
        })
    }
}

pub use auth::AuthClient;
pub use device_history::DeviceHistoryClient;
pub use device_live::DeviceLiveClient;
pub use forecast::ForecastClient;
pub use map::MapClient;
pub use metar::MetarClient;
pub use stations::StationsClient;
