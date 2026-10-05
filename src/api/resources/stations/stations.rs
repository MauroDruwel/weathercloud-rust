use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct StationsClient {
    pub http_client: HttpClient,
}

impl StationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns nearby stations as JSON (despite `text/html` content-type header).
    /// All `page/*` endpoints return `PageDevice` objects that include the **station name**.
    /// Values are scaled integers — divide by 10 (e.g. `temp: 281` = 28.1°C).
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
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.stations.get_nearby(1.1, 1.1, 1, None).await;
    /// }
    /// ```
    pub async fn get_nearby(
        &self,
        lat: f64,
        lon: f64,
        km: i64,
        options: Option<RequestOptions>,
    ) -> Result<PageDevicesResponse, ApiError> {
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
                &format!(
                    "page/coordinates/latitude/{}/longitude/{}/distance/{}",
                    lat, lon, km
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .stations
    ///         .get_popular(
    ///             &"BE".to_string(),
    ///             &GetPopularStationsRequestPeriod::Day,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_popular(
        &self,
        country: &str,
        period: &GetPopularStationsRequestPeriod,
        options: Option<RequestOptions>,
    ) -> Result<PageDevicesResponse, ApiError> {
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
                &format!("page/popular/country/{}/period/{}", country, period),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.stations.get_newest(&"BE".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_newest(
        &self,
        country: &str,
        options: Option<RequestOptions>,
    ) -> Result<PageDevicesResponse, ApiError> {
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
                &format!("page/newest/country/{}", country),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .stations
    ///         .get_most_followed(&"BE".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_most_followed(
        &self,
        country: &str,
        options: Option<RequestOptions>,
    ) -> Result<PageDevicesResponse, ApiError> {
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
                &format!("page/followers/country/{}", country),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.stations.get_last_views(None).await;
    /// }
    /// ```
    pub async fn get_last_views(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<PageDevicesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("X-Requested-With".to_string())
                .or_insert_with(|| "XMLHttpRequest".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, "page/lastviews", None, None, options)
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.stations.get_own(None).await;
    /// }
    /// ```
    pub async fn get_own(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<PageDevicesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("X-Requested-With".to_string())
                .or_insert_with(|| "XMLHttpRequest".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, "page/own", None, None, options)
            .await
    }

    /// The station **name** is not available from any JSON API endpoint.
    /// The easiest way to get it is to fetch the station's HTML page and extract
    /// the name from the `<title>` or `og:title` meta tag.
    ///
    /// **Example response title:**
    /// ```
    /// WeatherStation Skyline - Weathercloud | Global network of weather stations
    /// ```
    ///
    /// Strip everything from ` - Weathercloud` onward to get the clean station name.
    ///
    /// > This is a plain HTML page, not a JSON API. Use it for scraping only.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use weathercloud_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .stations
    ///         .get_station_page(&"deviceId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_station_page(
        &self,
        device_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("X-Requested-With".to_string())
                .or_insert_with(|| "XMLHttpRequest".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("d{}", device_id), None, None, options)
            .await
    }
}
