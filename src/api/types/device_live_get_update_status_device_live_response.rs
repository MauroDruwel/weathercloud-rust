pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUpdateStatusDeviceLiveResponse {
    /// Seconds since last update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<i64>,
    /// "1" = online, "2" = recently online, "3" = offline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Current server Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_time: Option<i64>,
}

impl GetUpdateStatusDeviceLiveResponse {
    pub fn builder() -> GetUpdateStatusDeviceLiveResponseBuilder {
        <GetUpdateStatusDeviceLiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUpdateStatusDeviceLiveResponseBuilder {
    update: Option<i64>,
    status: Option<String>,
    server_time: Option<i64>,
}

impl GetUpdateStatusDeviceLiveResponseBuilder {
    pub fn update(mut self, value: i64) -> Self {
        self.update = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn server_time(mut self, value: i64) -> Self {
        self.server_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetUpdateStatusDeviceLiveResponse`].
    pub fn build(self) -> Result<GetUpdateStatusDeviceLiveResponse, BuildError> {
        Ok(GetUpdateStatusDeviceLiveResponse {
            update: self.update,
            status: self.status,
            server_time: self.server_time,
        })
    }
}
