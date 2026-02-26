use crate::error::EngineError;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tungstenite::Utf8Bytes;

pub struct SmartSwitch {
    pub url: String,
    pub query: Value,
    pub current_state: AtomicBool,
    pub set_state: AtomicBool,
}

impl SmartSwitch {
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
        }
        Ok(())
    }
}
