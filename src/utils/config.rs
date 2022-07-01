use envy;
use once_cell::sync::Lazy;
use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(Debug, Deserialize, SmartDefault)]
pub struct AppConfig {
    #[default = "~/dirmarks.db"]
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let config = match envy::prefixed("DM_").from_env::<AppConfig>() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("{:?}", e);
                Self::default()
            },
        };
        debug!("{:?}", config);
        config
    }
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(AppConfig::from_env);
