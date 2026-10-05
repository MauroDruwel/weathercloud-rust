pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceProfileObserver {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}

impl DeviceProfileObserver {
    pub fn builder() -> DeviceProfileObserverBuilder {
        <DeviceProfileObserverBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceProfileObserverBuilder {
    name: Option<String>,
    nickname: Option<String>,
    company: Option<String>,
}

impl DeviceProfileObserverBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeviceProfileObserver`].
    pub fn build(self) -> Result<DeviceProfileObserver, BuildError> {
        Ok(DeviceProfileObserver {
            name: self.name,
            nickname: self.nickname,
            company: self.company,
        })
    }
}
