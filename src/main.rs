mod config;
mod error;

use crate::config::{AppConfig, Cli};
use crate::error::AppError;
use clap::Parser;
use log::info;
use serde_json::json;
use smarthy_engine::switch::SmartSwitch;
use std::sync::atomic::AtomicBool;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    let config = AppConfig::parse_config(&cli.config)?;

    let env = EnvFilter::new(format!(
        "smarthy={},{}",
        config.app_log_level, config.all_log_level
    ));

    let timer = tracing_subscriber::fmt::time::LocalTime::rfc_3339();
    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_target(true)
        .with_env_filter(env)
        .init();

    info!("Config used: {}\n{config:#?}", cli.config);

    let mut x = SmartSwitch {
        url: "ws://192.168.13.66/rpc".to_string(),
        query: json!({
            "id": 1,
            "src": "rust-client",
            "method": "Temperature.GetStatus",
            "params": { "id": 101 }
        }),
        current_state: AtomicBool::new(false),
        set_state: AtomicBool::new(false),
    };

    x.poll_state().await?;
    println!("Hello, world!");
    Ok(())
}
