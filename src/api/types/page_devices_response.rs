pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageDevicesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<PageDevice>>,
}

impl PageDevicesResponse {
    pub fn builder() -> PageDevicesResponseBuilder {
        <PageDevicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageDevicesResponseBuilder {
    devices: Option<Vec<PageDevice>>,
}

impl PageDevicesResponseBuilder {
    pub fn devices(mut self, value: Vec<PageDevice>) -> Self {
        self.devices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageDevicesResponse`].
    pub fn build(self) -> Result<PageDevicesResponse, BuildError> {
        Ok(PageDevicesResponse {
            devices: self.devices,
        })
    }
}
