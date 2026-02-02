//! Settings module for kakeboor
//!
//! This module provides environment-specific settings configuration using TOML files.

use reinhardt::conf::settings::builder::SettingsBuilder;
use reinhardt::conf::settings::profile::Profile;
use reinhardt::conf::settings::sources::{DefaultSource, LowPriorityEnvSource, TomlFileSource};
use reinhardt::Settings;
use std::env;

/// Get settings based on environment variable
pub fn get_settings() -> Settings {
    let profile_str = env::var("REINHARDT_ENV").unwrap_or_else(|_| "local".to_string());
    let profile = Profile::parse(&profile_str);

    let base_dir = env::current_dir().expect("Failed to get current directory");
    let settings_dir = base_dir.join("settings");

    let merged = SettingsBuilder::new()
        .profile(profile)
        .add_source(
            DefaultSource::new()
                .with_value("base_dir", serde_json::Value::String(base_dir.display().to_string()))
                .with_value("debug", serde_json::Value::Bool(false))
                .with_value("language_code", serde_json::Value::String("en-us".to_string()))
                .with_value("time_zone", serde_json::Value::String("UTC".to_string()))
                .with_value("use_i18n", serde_json::Value::Bool(true))
                .with_value("use_tz", serde_json::Value::Bool(true))
                .with_value("append_slash", serde_json::Value::Bool(true))
                .with_value("default_auto_field", serde_json::Value::String("BigAutoField".to_string())),
        )
        .add_source(LowPriorityEnvSource::new().with_prefix("REINHARDT_"))
        .add_source(TomlFileSource::new(settings_dir.join("base.toml")))
        .add_source(TomlFileSource::new(settings_dir.join(format!("{}.toml", profile_str))))
        .build()
        .expect("Failed to build settings");

    merged
        .into_typed()
        .expect("Failed to convert settings to Settings struct")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_settings() {
        let settings = get_settings();
        assert!(!settings.secret_key.is_empty());
    }
}
