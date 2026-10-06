use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct MetarClient {
    pub http_client: HttpClient,
}

impl MetarClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Most `device/*` routes work identically for METAR (airport) stations by replacing the `device` prefix with `metar`.
    ///
    /// METAR station IDs are **ICAO codes** (4 letters), e.g. `EBBR` for Brussels Airport.
    ///
    /// **Supported `metar/*` routes (same request/response as their `device/*` counterparts):**
    /// - `GET /metar/values/{icao}` — current readings
    /// - `GET /metar/stats?code={icao}` — statistics
    /// - `GET /metar/wind?code={icao}` — wind rose
    /// - `GET /metar/info/{icao}` — station metadata
    /// - `POST /metar/ajaxupdatedate` — last update time
    /// - `POST /metar/ajaxprofile` — station profile
    /// - `POST /metar/evolution` — time-series history
    ///
    /// > **Not supported for METAR:** `/device/ajaxdevicestats`
    ///
    /// # Arguments
    ///
    /// * `device_id` - ICAO airport code
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = WeathercloudClient::new(config).expect("Failed to build client");
    ///     client.metar.get_values(&"EBBR".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_values(
        &self,
        device_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeviceValues, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("X-Requested-With".to_string())
                .or_insert_with(|| "XMLHttpRequest".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("metar/values/{}", device_id),
                None,
                None,
                options,
            )
            .await
    }
}
