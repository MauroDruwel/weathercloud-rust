use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct MapClient {
    pub http_client: HttpClient,
}

impl MapClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns stations visible on the map for a given location bounding box.
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
    ///         .map
    ///         .get_devices(
    ///             &GetDevicesMapRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_devices(
        &self,
        request: &GetDevicesMapRequest,
        options: Option<RequestOptions>,
    ) -> Result<MapDevicesResponse, ApiError> {
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
                "map/devices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

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
    ///         .map
    ///         .get_background_devices(
    ///             &GetBackgroundDevicesMapRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_background_devices(
        &self,
        request: &GetBackgroundDevicesMapRequest,
        options: Option<RequestOptions>,
    ) -> Result<MapDevicesResponse, ApiError> {
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
                "map/bgdevices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

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
    ///         .map
    ///         .get_metars(
    ///             &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_metars(
        &self,
        request: &HashMap<String, serde_json::Value>,
        options: Option<RequestOptions>,
    ) -> Result<GetMetarsMapResponse, ApiError> {
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
                "map/metars",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
