use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DeviceLiveClient {
    pub http_client: HttpClient,
}

impl DeviceLiveClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the latest sensor values for a device. **No CSRF token needed.**
    /// This is the primary endpoint for a Home Assistant sensor integration.
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
    ///         .device_live
    ///         .get_values(&"5726468552".to_string(), None)
    ///         .await;
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
                &format!("device/values/{}", device_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns current values plus day/month/year min and max for all sensors.
    /// Each value is a `[unix_timestamp, value]` tuple.
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
    ///         .device_live
    ///         .get_stats(
    ///             &GetStatsQueryRequest {
    ///                 code: "5726468552".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_stats(
        &self,
        request: &GetStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeviceStats, ApiError> {
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
                "device/stats",
                None,
                QueryBuilder::new()
                    .string("code", request.code.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Station name, location, elevation, equipment info.
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
    ///         .device_live
    ///         .get_info(&"5726468552".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_info(
        &self,
        device_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeviceInfo, ApiError> {
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
                &format!("device/info/{}", device_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Wind direction distribution data for the wind rose chart.
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
    ///         .device_live
    ///         .get_wind_rose(
    ///             &GetWindRoseQueryRequest {
    ///                 code: "5726468552".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_wind_rose(
        &self,
        request: &GetWindRoseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WindData, ApiError> {
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
                "device/wind",
                None,
                QueryBuilder::new()
                    .string("code", request.code.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns seconds since last update and device online status.
    ///
    /// > ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.
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
    ///         .device_live
    ///         .get_update_status(
    ///             &GetUpdateStatusDeviceLiveRequest {
    ///                 d: "5726468552".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_update_status(
        &self,
        request: &GetUpdateStatusDeviceLiveRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetUpdateStatusDeviceLiveResponse, ApiError> {
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
                "device/ajaxupdatedate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns observer name, follower count, and device brand/model.
    ///
    /// > ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.
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
    ///         .device_live
    ///         .get_owner_profile(
    ///             &GetOwnerProfileDeviceLiveRequest {
    ///                 d: "5726468552".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_owner_profile(
        &self,
        request: &GetOwnerProfileDeviceLiveRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeviceProfile, ApiError> {
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
                "device/ajaxprofile",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
