use std::iter::zip;
use crate::config::SmartDevices;
use crate::device::{DeviceConnectionHandles, DeviceConnections, SmartDevice};
use crate::error::EngineError;
use crate::message::{ShellyRawMessage, SmartDeviceMessage};
use futures_util::StreamExt;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::accept_async;
use tracing::{debug, error, info};
use tungstenite::Message;

pub struct WebSocketServer {
    pub listen_addr: SocketAddr,
    pub devices: SmartDevices,
    pub handles: DeviceConnectionHandles,
}

impl WebSocketServer {
    pub fn new(listen_addr: SocketAddr, devices: SmartDevices, handles: DeviceConnectionHandles) -> Result<Self, EngineError> {
        Ok(Self {
            listen_addr,
            devices,
            handles,
        })
    }
    pub async fn serve(&self) -> Result<(), EngineError> {
        let listener = TcpListener::bind(self.listen_addr).await?;
        info!(
            "Shelly WebSocket server listening on ws://{}",
            self.listen_addr
        );
        while let Ok((stream, _)) = listener.accept().await {
            let devices = self.devices.clone();
            let handles = self.handles.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, devices, handles).await {
                    error!("{e}")
                }
            });
        }
        Ok(())
    }

    async fn handle_connection(stream: tokio::net::TcpStream, devices: SmartDevices, handles: DeviceConnectionHandles) -> Result<(), EngineError> {
        let ws_stream = accept_async(stream).await?;
        let (sender, mut receiver) = ws_stream.split();
        debug!("New connection!");
        if let Some(Ok(Message::Text(text))) = receiver.next().await {
            let data = serde_json::from_str::<serde_json::Value>(&text)?;

            debug!("Data: {:#?}", data);
            let msg: ShellyRawMessage = serde_json::from_str(&text)?;
            debug!("Raw Message: {:#?}", msg);
            let device = SmartDeviceMessage::from(msg);
            debug!("Message: {:#?}", device);
            let (writes_sender, writes_receiver) = mpsc::channel(100);
            let (reads_sender, reads_receiver) = broadcast::channel(100);
            let d = SmartDevice::from_message(device);
            debug!("Device: {:#?}", d);
            let id = d.id.clone();
            let id_local = d.id.clone();
            let writer_task = tokio::spawn(async move {
               SmartDevice::write_worker(id_local, sender, writes_receiver).await;
            });

            let writer = writes_sender.clone();
            let id_local = d.id.clone();
            tokio::spawn(async move {
               if let Err(e) = d.read_worker(id_local, receiver, reads_sender, writer).await {
                   writer_task.abort();
                   error!("Reader Error: {e}")
               }
            });

            let connections = DeviceConnections::new(writes_sender, reads_receiver);
            // TODO cleanup old connections
            info!("Device {id} connected");
            handles.write().await.insert(id, connections);
        }
        Ok(())
    }
}
