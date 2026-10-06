use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DeviceHistoryClient {
    pub http_client: HttpClient,
}

impl DeviceHistoryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns hourly aggregated history for a given variable and period.
    ///
    /// > ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.
    ///
    /// **Variable codes:**
    /// | Code | Sensor |
    /// |------|--------|
    /// | 101  | Temperature (°C) |
    /// | 201  | Humidity (%) |
    /// | 541  | Dew point (°C) |
    /// | 641  | Barometric pressure (hPa) |
    /// | 701  | Wind speed (m/s) |
    /// | 6001 | Wind direction (°) |
    /// | 6501 | Wind gust / high speed (m/s) |
    /// | 801  | Rain (mm) |
    /// | 811  | Rain rate (mm/h) |
    /// | 1001 | Solar radiation (W/m²) |
    /// | 1101 | UV index |
    ///
    /// **Period values:** `day`, `week`, `month`, `year`
    ///
    /// # Arguments
    ///
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
    ///     client
    ///         .device_history
    ///         .get_evolution(
    ///             &GetEvolutionDeviceHistoryRequest {
    ///                 device: "5726468552".to_string(),
    ///                 variable: 101,
    ///                 period: GetEvolutionDeviceHistoryRequestPeriod::Day,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_evolution(
        &self,
        request: &GetEvolutionDeviceHistoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EvolutionResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("X-Requested-With".to_string())
                .or_insert_with(|| "XMLHttpRequest".to_string());
            Some(o)
        };
        self.http_client
            .execute_form_request(
                Method::POST,
                "device/evolution",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
