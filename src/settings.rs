use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub max_sessions: u64,
    pub time_of_cookies_s: u64,
    pub time_of_polling_items: u64,
    pub db_name: String,
    pub secure: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_builder = Config::builder()
            .add_source(File::with_name("./config/settings.toml").required(false))
            .add_source(File::with_name("./config/settings.toml").required(false))
            .add_source(File::with_name("./config/settings.yml").required(false))
            .add_source(File::with_name("./config/settings.json").required(false))
            .add_source(Environment::default().separator("__"));

        config_builder.build().unwrap().try_deserialize()
    }
}
