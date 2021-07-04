use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub max_sessions: u64,
    pub time_of_cookies_s: u64,
    pub db_name: String,
    pub secure: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("./config/settings.toml").required(false))?;
        s.merge(File::with_name("./config/settings.yml").required(false))?;
        s.merge(File::with_name("./config/settings.json").required(false))?;

        s.merge(Environment::new().separator("__"))?;

        s.try_into()
    }
}
