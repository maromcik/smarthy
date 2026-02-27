use tracing::error;
use crate::config::{SmartDevices, WsConfig};
use crate::device::DeviceConnectionHandles;
use crate::error::EngineError;
use crate::ws::WebSocketServer;

pub mod error;
pub mod switch;
pub mod ws;
mod message;
mod device;
pub mod config;
mod engine;

pub async fn init(config: WsConfig, devices: SmartDevices) -> Result<(), EngineError> {
    let handles = DeviceConnectionHandles::default();
    let ws = WebSocketServer::new(config.listen, devices, handles.clone())?;
    let task = tokio::spawn(async move {
        if let Err(e) = ws.serve().await {
            error!("{e}")
        }
    });

    task.await;

    Ok(())
}