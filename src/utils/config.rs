use log::{debug, error};
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
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("DM"))
            .build();
        if let Ok(config) = config {
            let result = config.try_deserialize();
            return match result {
                Ok(config) => {
                    debug!("{:?}", config);
                    config
                },
                Err(err) => {
                    error!("{:?}", err);
                    Self::default()
                },
            };
        }
        Self::default()
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::from_env);
