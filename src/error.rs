use serde::{Deserialize, Serialize};
use smarthy_engine::error::EngineError;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Error)]
pub enum AppError {
    #[error("EngineError -> {0}")]
    EngineError(EngineError),
    #[error("Serialize/Deserialize error: {0}")]
    SerdeError(String),
    #[error("config error: {0}")]
    ConfigError(String),
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self)
    }
}

impl From<EngineError> for AppError {
    fn from(e: EngineError) -> Self {
        Self::EngineError(e)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        Self::ConfigError(e.to_string())
    }
}
