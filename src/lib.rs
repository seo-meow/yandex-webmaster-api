//! # Yandex Webmaster API Client
//!
//! This crate provides a Rust client for the Yandex Webmaster API.
//!
//! ## Example
//!
//! ```no_run
//! use yandex_webmaster_api::YandexWebmasterClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = YandexWebmasterClient::new("your-oauth-token".to_string()).await?;
//!     println!("User ID: {}", client.user_id());
//!     Ok(())
//! }
//! ```

mod client;
mod dto;
mod error;
mod middleware;

pub use client::YandexWebmasterClient;
pub use dto::*;
pub use error::{Result, YandexWebmasterError};
