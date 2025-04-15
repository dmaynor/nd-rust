use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: Option<bool>,
    pub database: Option<DatabaseSettings>,
    pub log_level: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config_path = std::env::var("ND_CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());

        let s = config::Config::builder()
            // Start off by merging in the default configuration file
            .add_source(config::File::with_name(&config_path).required(false))
            // Add in settings from the environment (with a prefix of ND)
            // Eg.. `ND_DATABASE__URL=...` would set the `database.url` field
            .add_source(config::Environment::with_prefix("nd").separator("__")) // Use double underscore for nested
            .build()?;

        s.try_deserialize()
    }
}

mod discovery;
pub use discovery::{DiscoveryJob, DiscoveryResult, DiscoveryManager};
