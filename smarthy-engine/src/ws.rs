use std::net::SocketAddr;
use futures_util::{StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tracing::info;
use tungstenite::{Message};
use crate::device::SmartDevice;
use crate::error::EngineError;
use crate::message::ShellyMessage;

pub struct WebSocketServer {
    pub listen_addr: SocketAddr,
}

impl WebSocketServer {

    pub fn new(listen_addr: &str) -> Result<Self, EngineError> {
        Ok(Self { listen_addr: listen_addr.parse()? })
    }
    pub async fn serve(&self) -> Result<(), EngineError> {
        let listener = TcpListener::bind(self.listen_addr).await?;
        println!("Shelly WebSocket server listening on ws://{}", self.listen_addr);
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(stream));
        }
        Ok(())
    }

    async fn handle_connection(stream: tokio::net::TcpStream) -> Result<(), EngineError> {
        let ws_stream = accept_async(stream).await?;
        let (mut sender, mut receiver) = ws_stream.split();
        info!("Shelly connected");

        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(text) = message {
                let data = serde_json::from_str::<serde_json::Value>(&text)?;
                let msg: ShellyMessage = serde_json::from_str(&text)?;
                let device = SmartDevice::from(msg);

                info!("Data: {:#?}", data);
                info!("Device: {:#?}", device);
            }
        }
        Ok(())
    }
}