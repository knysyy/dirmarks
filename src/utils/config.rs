use config::{Config, Environment};
use once_cell::sync::Lazy;
use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(Debug, Deserialize, SmartDefault)]
pub struct AppConfig {
    #[default = "./dm.db"]
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let cfg = Config::builder()
            .add_source(Environment::with_prefix("DM"))
            .build();

        match cfg {
            Ok(config) => {
                // TODO avoid using unwrap
                let app_config: AppConfig = config.try_deserialize().unwrap();
                debug!("{:?}", app_config);
                app_config
            }
            Err(e) => {
                debug!("{:?}", e);
                Self::default()
            }
        }
    }
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(AppConfig::from_env);
