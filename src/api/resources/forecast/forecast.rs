use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ForecastClient {
    pub http_client: HttpClient,
}

impl ForecastClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    ///         .forecast
    ///         .get_daily(
    ///             &GetDailyQueryRequest {
    ///                 id: "5726468552".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_daily(
        &self,
        request: &GetDailyQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ForecastResponse, ApiError> {
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
                "forecast/daily",
                None,
                QueryBuilder::new().string("id", request.id.clone()).build(),
                options,
            )
            .await
    }
}
