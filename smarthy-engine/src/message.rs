use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::error;
use tungstenite::{Message, Utf8Bytes};
use crate::error::EngineError;

pub type TemperatureMessages = HashMap<String, TemperatureMessage>;
pub type SwitchMessages = HashMap<String, SwitchMessage>;
pub type InputMessages = HashMap<String, InputMessage>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageType {
    NotifyStatus,
    NotifyFullStatus,
    Empty,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShellyRawMessage {
    #[serde(rename = "src")]
    pub id: String,
    pub method: MessageType,
    pub params: Params,
}

impl ShellyRawMessage {
    pub fn from(bytes: Utf8Bytes) -> Result<Self, EngineError> {
        match serde_json::from_str(&bytes) {
            Ok(msg) => Ok(msg),
            Err(e) => {
                error!("Invalid text received: {bytes}: Error: {e}");
                Err(EngineError::from(e))
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Params {
    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

impl Params {
    pub fn temperatures(&self) -> HashMap<String, TemperatureMessage> {
        self.extract_prefixed("temperature:")
    }

    pub fn switches(&self) -> HashMap<String, SwitchMessage> {
        self.extract_prefixed("switch:")
    }

    pub fn inputs(&self) -> HashMap<String, InputMessage> {
        self.extract_prefixed("input:")
    }

    fn extract_prefixed<T>(&self, prefix: &str) -> HashMap<String, T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.rest
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k, serde_json::from_value(v.clone()).ok()))
            .filter_map(|(k, v)| v.map(|v| (k.clone(), v)))
            .collect::<HashMap<String, T>>()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ShellyUpdateMessage {
    FullStatus(SmartDeviceMessage),
    Temperature(TemperatureMessage),
    Switch(SwitchMessage),
    Input(InputMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SendMessage {
    Switch(SwitchMessage),
    Pong,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmartDeviceMessage {
    pub id: String,
    pub method: MessageType,
    pub temperatures: TemperatureMessages,
    pub switches: SwitchMessages,
    pub inputs: InputMessages,
}

impl From<ShellyRawMessage> for SmartDeviceMessage {
    fn from(msg: ShellyRawMessage) -> Self {
        let temperatures = msg.params.temperatures();
        let switches = msg.params.switches();
        let inputs = msg.params.inputs();

        let method = if inputs.is_empty() && switches.is_empty() && temperatures.is_empty() {
            MessageType::Empty
        } else {
            msg.method
        };
        Self {
            id: msg.id,
            method,
            temperatures,
            switches,
            inputs,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemperatureMessage {
    #[serde(rename = "tC")]
    pub tc: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SwitchMessage {
    pub output: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InputMessage {
    pub state: bool,
}