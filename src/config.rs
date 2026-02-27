use std::collections::HashMap;
use crate::error::AppError;
use clap::Parser;
use config::Config;
use serde::{Deserialize, Serialize};
use smarthy_engine::config::{SmartDeviceConfig, SmartDevices, WsConfig};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short,
        long,
        value_name = "CONFIG_FILE",
        default_value = "smarthy.yaml",
        env = "SMARTHY_CONFIG_FILE"
    )]
    pub config: String,
}

fn default_log_level() -> String { "info".to_string() }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AppConfig {
    #[serde(default = "default_log_level")]
    pub app_log_level: String,
    #[serde(default = "default_log_level")]
    pub all_log_level: String,
    pub ws_config: WsConfig,
    #[serde(default)]
    pub devices: SmartDevices,
}

impl AppConfig {
    pub fn parse_config(settings_path: &str) -> Result<AppConfig, AppError> {
        let settings = Config::builder()
            .add_source(config::File::with_name(settings_path))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        let config = settings.try_deserialize::<AppConfig>()?;

        Ok(config)
    }
}
