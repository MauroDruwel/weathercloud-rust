//! # Weathercloud Unofficial API SDK
//!
//! The official Rust SDK for the Weathercloud Unofficial API.
//!
//! ## Getting Started
//!
//! ```rust
//! use weathercloud_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         ..Default::default()
//!     };
//!     let client = ApiClient::new(config).expect("Failed to build client");
//!     client
//!         .auth
//!         .login(
//!             &LoginAuthRequest {
//!                 login_form_entity: "LoginForm[entity]".to_string(),
//!                 login_form_password: "LoginForm[password]".to_string(),
//!                 login_form_remember_me: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
