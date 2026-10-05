//! API client and types for the Weathercloud Unofficial API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    ApiClient, AuthClient, DeviceHistoryClient, DeviceLiveClient, ForecastClient, MapClient,
    MetarClient, StationsClient,
};
pub use types::*;
