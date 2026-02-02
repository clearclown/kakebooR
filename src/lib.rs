//! kakebooR - Household Budget Application
//!
//! A REST API for managing household finances built with Reinhardt.

pub mod config;
pub mod apps;

// Re-export commonly used items
pub use config::settings::get_settings;
pub use config::urls::url_patterns;
