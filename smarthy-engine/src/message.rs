use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ptr::hash;

pub type Temperatures = HashMap<String, Temperature>;
pub type Switches = HashMap<String, Switch>;
pub type Inputs = HashMap<String, Input>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShellyMessage {
    pub method: String,
    pub src: String,
    pub params: Params,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Params {
    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

impl Params {
    pub fn temperatures(&self) -> HashMap<String, Temperature> {
        self.extract_prefixed("temperature:")
    }

    pub fn switches(&self) -> HashMap<String, Switch> {
        self.extract_prefixed("switch:")
    }

    pub fn inputs(&self) -> HashMap<String, Input> {
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
pub struct Temperature {
    pub id: u32,
    #[serde(rename = "tC")]
    pub tc: f64,
    #[serde(rename = "tF")]
    pub tf: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Switch {
    pub output: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Input {
    pub id: u32,
    pub state: bool,
}