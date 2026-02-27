use crate::message::{Inputs, ShellyMessage, Switches, Temperatures};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmartDevice {
    pub id: String,
    pub temperatures: Temperatures,
    pub switches: Switches,
    pub inputs: Inputs,
}

impl From<ShellyMessage> for SmartDevice {
    fn from(msg: ShellyMessage) -> Self {
        Self {
            id: msg.src,
            temperatures: msg.params.temperatures(),
            switches: msg.params.switches(),
            inputs: msg.params.inputs(),
        }
    }
}
