use once_cell::sync::Lazy;
use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Config {
    #[default = "./dm.db"]
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let mut cfg = config::Config::new();
        let result = cfg.merge(config::Environment::with_prefix("DM"));
        if result.is_ok() {
            let result = cfg.try_into();
            return match result {
                Ok(config) => {
                    debug!("{:?}", config);
                    config
                },
                Err(err) => {
                    debug!("{:?}", err);
                    Self::default()
                }
            }
        }
        Self::default()
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_env());
