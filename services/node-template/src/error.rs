use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Blockchain connection error: {0}")]
    BlockchainError(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Staking failed: {0}")]
    StakingError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Metrics error: {0}")]
    MetricsError(#[from] prometheus::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for NodeError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            NodeError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service temporarily unavailable".to_string(),
            ),
            NodeError::BlockchainError(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("Blockchain connection error: {msg}"),
            ),
            NodeError::InvalidAddress(addr) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid address: {addr}"),
            ),
            NodeError::InsufficientBalance => (
                StatusCode::BAD_REQUEST,
                "Insufficient balance for staking".to_string(),
            ),
            NodeError::StakingError(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Staking operation failed: {msg}"),
            ),
            NodeError::ConfigError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Configuration error: {msg}"),
            ),
            NodeError::MetricsError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Metrics error: {err}"),
            ),
            NodeError::Utf8Error(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("UTF-8 conversion error: {err}"),
            ),
            NodeError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
