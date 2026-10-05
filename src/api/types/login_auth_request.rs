pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginAuthRequest {
    /// Username or email address
    #[serde(rename = "LoginForm[entity]")]
    #[serde(default)]
    pub login_form_entity: String,
    /// Account password
    #[serde(rename = "LoginForm[password]")]
    #[serde(default)]
    pub login_form_password: String,
    /// Keep the user logged in
    #[serde(rename = "LoginForm[rememberMe]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_form_remember_me: Option<LoginAuthRequestLoginFormRememberMe>,
}

impl LoginAuthRequest {
    pub fn builder() -> LoginAuthRequestBuilder {
        <LoginAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginAuthRequestBuilder {
    login_form_entity: Option<String>,
    login_form_password: Option<String>,
    login_form_remember_me: Option<LoginAuthRequestLoginFormRememberMe>,
}

impl LoginAuthRequestBuilder {
    pub fn login_form_entity(mut self, value: impl Into<String>) -> Self {
        self.login_form_entity = Some(value.into());
        self
    }

    pub fn login_form_password(mut self, value: impl Into<String>) -> Self {
        self.login_form_password = Some(value.into());
        self
    }

    pub fn login_form_remember_me(mut self, value: LoginAuthRequestLoginFormRememberMe) -> Self {
        self.login_form_remember_me = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoginAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`login_form_entity`](LoginAuthRequestBuilder::login_form_entity)
    /// - [`login_form_password`](LoginAuthRequestBuilder::login_form_password)
    pub fn build(self) -> Result<LoginAuthRequest, BuildError> {
        Ok(LoginAuthRequest {
            login_form_entity: self
                .login_form_entity
                .ok_or_else(|| BuildError::missing_field("login_form_entity"))?,
            login_form_password: self
                .login_form_password
                .ok_or_else(|| BuildError::missing_field("login_form_password"))?,
            login_form_remember_me: self.login_form_remember_me,
        })
    }
}
