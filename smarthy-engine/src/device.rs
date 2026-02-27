use crate::message::{
    InputMessage, SendMessage, ShellyRawMessage, ShellyUpdateMessage, SmartDeviceMessage,
    SwitchMessage, TemperatureMessage,
};
use atomic_float::AtomicF64;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, broadcast, mpsc};
use tokio_tungstenite::WebSocketStream;
use tracing::{debug, error, info, trace, warn};
use tungstenite::{Bytes, Error, Message, Utf8Bytes};

pub type DeviceConnectionHandles = Arc<RwLock<HashMap<String, DeviceConnections>>>;

pub struct DeviceConnections {
    pub writes_sender: mpsc::Sender<SendMessage>,
    pub reads_receiver: broadcast::Receiver<SmartDeviceMessage>,
}

impl DeviceConnections {
    pub fn new(
        writes_sender: mpsc::Sender<SendMessage>,
        reads_receiver: broadcast::Receiver<SmartDeviceMessage>,
    ) -> Self {
        Self {
            writes_sender,
            reads_receiver,
        }
    }
}

#[derive(Debug, Default)]
pub struct SmartDevice {
    pub id: String,
    pub temperatures: HashMap<String, SmartTemperature>,
    pub switches: HashMap<String, SmartSwitch>,
    pub inputs: HashMap<String, SmartInput>,
}

impl SmartDevice {
    pub fn from_message(msg: SmartDeviceMessage) -> Self {
        Self {
            id: msg.id,
            temperatures: msg
                .temperatures
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            switches: msg
                .switches
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            inputs: msg.inputs.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }

    pub async fn read_worker(
        self,
        id: String,
        mut receiver: SplitStream<WebSocketStream<TcpStream>>,
        sender: broadcast::Sender<SmartDeviceMessage>,
        writes_sender: mpsc::Sender<SendMessage>,
    ) -> Result<(), Error> {
        while let Some(data) = receiver.next().await {
            match data? {
                Message::Text(text) => {
                    debug!("Raw update received {text}");
                    let Ok(msg) = ShellyRawMessage::from(text) else {
                        continue;
                    };
                    info!("Message: {msg:?}");
                    let device = SmartDeviceMessage::from(msg);
                    info!("Device: {device:?}");
                    if let Err(e) = sender.send(device) {
                        error!("Error sending message: {}", e);
                    }
                }
                Message::Ping(msg) => {
                    trace!("Ping received: {msg:?}");
                    if let Err(e) = writes_sender.send(SendMessage::Pong).await {
                        error!("Error sending pong via the channel: {}", e);
                    }
                }
                msg => {
                    warn!("Unhandled message: {msg:?}");
                }
            }
        }
        info!("Device {id} disconnected");
        Ok(())
    }

    pub async fn write_worker(
        id: String,
        mut sender: SplitSink<WebSocketStream<TcpStream>, Message>,
        mut receiver: mpsc::Receiver<SendMessage>,
    ) {
        while let Some(msg) = receiver.recv().await {
            let message = match msg {
                SendMessage::Switch(msg) => match serde_json::to_string(&msg) {
                    Ok(text) => Message::Text(Utf8Bytes::from(text)),
                    Err(e) => {
                        error!("Error serializing message: {}", e);
                        continue;
                    }
                },
                SendMessage::Pong => {
                    debug!("Pong sent");
                    Message::Pong(Bytes::new())
                }
            };

            if let Err(e) = sender.send(message).await {
                error!("Error sending message: {}", e);
            }
            debug!("Message sent");
        }
    }
}

#[derive(Debug, Default)]
pub struct SmartSwitch {
    pub state: AtomicBool,
}

impl From<SwitchMessage> for SmartSwitch {
    fn from(msg: SwitchMessage) -> Self {
        Self {
            state: AtomicBool::new(msg.output),
        }
    }
}

#[derive(Debug, Default)]
pub struct SmartInput {
    pub state: AtomicBool,
}

impl From<InputMessage> for SmartInput {
    fn from(msg: InputMessage) -> Self {
        Self {
            state: AtomicBool::new(msg.state),
        }
    }
}

#[derive(Debug, Default)]
pub struct SmartTemperature {
    pub tc: AtomicF64,
}

impl From<TemperatureMessage> for SmartTemperature {
    fn from(msg: TemperatureMessage) -> Self {
        Self {
            tc: AtomicF64::new(msg.tc),
        }
    }
}
