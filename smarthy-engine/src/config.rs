use std::collections::HashMap;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

pub type SmartDevices = HashMap<String, SmartDeviceConfig>; 
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SmartDeviceConfig {
    pub temperatures: Vec<String>,
    pub switches: Vec<String>,
    pub inputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct WsConfig {
    pub listen: SocketAddr,
}