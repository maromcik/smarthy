use crate::error::EngineError;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};
use tungstenite::Utf8Bytes;

pub struct SmartSwitch {
    pub url: String,
    pub query: Value,
    pub current_state: AtomicBool,
    pub set_state: AtomicBool,
}

impl SmartSwitch {

    pub async fn expose(&self) -> Result<(), EngineError> {
        let addr = "0.0.0.0:8080";  // Listen on all interfaces
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Shelly WebSocket server listening on ws://{}", addr);
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(self.query.to_string().clone(), stream));
        }
        Ok(())
    }
    pub async fn handle_connection(q: String, stream: tokio::net::TcpStream) -> Result<(), EngineError> {
        let ws_stream = accept_async(stream).await?;
        let (mut sender, mut receiver) = ws_stream.split();

        println!("New Shelly device connected!");

        // Listen for messages
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(text) = message {
                println!("Received: {}", text);
            }
            sender.send(Message::Text(Utf8Bytes::from(q.clone())))
                .await?;
            sleep(std::time::Duration::from_secs(5)).await;
        }
        Ok(())

    }
    pub async fn poll_state(&mut self) -> Result<(), EngineError> {
        let (ws_stream, _) = connect_async(self.url.as_str()).await?;
        let (mut write, mut read) = ws_stream.split();

        write
            .send(Message::Text(Utf8Bytes::from(self.query.to_string())))
            .await?;

        while let Some(msg) = read.next().await {
            let msg = msg?;
            match msg {
                Message::Text(text) => {
                    let parsed: serde_json::Value = serde_json::from_str(&text)?;
                    println!("Parsed: {:#?}", parsed);
                }
                _ => {}
            }
            write
                .send(Message::Text(Utf8Bytes::from(self.query.to_string())))
                .await?;
            sleep(std::time::Duration::from_secs(5)).await;
        }
        Ok(())
    }
}
