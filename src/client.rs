//! WASM client module for kakebooR
//!
//! This module contains the frontend code that runs in the browser.

#[cfg(target_arch = "wasm32")]
pub mod api;
#[cfg(target_arch = "wasm32")]
pub mod components;
#[cfg(target_arch = "wasm32")]
pub mod lib;
#[cfg(target_arch = "wasm32")]
pub mod pages;
#[cfg(target_arch = "wasm32")]
pub mod router;

#[cfg(target_arch = "wasm32")]
pub use lib::{init_global_router, with_router};
