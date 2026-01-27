//! kakeboor library
//!
//! This is the main library crate for kakeboor.

// Shared types (available for both WASM and server)
pub mod shared;

// Client module (WASM only)
#[cfg(target_arch = "wasm32")]
pub mod client;

// Server modules (native only)
#[cfg(not(target_arch = "wasm32"))]
pub mod apps;
#[cfg(not(target_arch = "wasm32"))]
pub mod config;

// Re-export commonly used items (native only)
#[cfg(not(target_arch = "wasm32"))]
pub use config::settings::get_settings;
#[cfg(not(target_arch = "wasm32"))]
pub use config::urls::url_patterns;

// Re-export Reinhardt components for convenience (native only)
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_core;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_http;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_server;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_urls;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_views;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_di;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_conf;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_db;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_rest;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_macros;
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_routers_macros;

// Re-export hyper::Method at crate root for macro compatibility (native only)
#[cfg(not(target_arch = "wasm32"))]
pub use hyper::Method;
