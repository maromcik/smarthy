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
use smarthy_engine::ws::WebSocketServer;

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

    let ws = WebSocketServer::new("0.0.0.0:8080")?;
    ws.serve().await?;

    Ok(())
}
