use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Error)]
pub enum EngineError {
    #[error("I/O Error: {0}")]
    IOError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Serialize/Deserialize error: {0}")]
    SerdeError(String),
}

impl Debug for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for EngineError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        EngineError::WebSocketError(err.to_string())
    }
}

impl From<serde_json::Error> for EngineError {
    fn from(err: serde_json::Error) -> Self {
        EngineError::SerdeError(err.to_string())
    }
}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::IOError(err.to_string())
    }
}

impl From<std::net::AddrParseError> for EngineError {
    fn from(err: std::net::AddrParseError) -> Self {
        EngineError::ParseError(err.to_string())
    }
}