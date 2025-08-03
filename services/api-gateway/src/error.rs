use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Service registry error: {0}")]
    ServiceRegistryError(String),
    
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::ServiceNotFound(service) => (
                StatusCode::NOT_FOUND,
                format!("Service '{}' not found", service),
            ),
            ApiError::ServiceRegistryError(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Service registry error: {}", msg),
            ),
            ApiError::RequestError(err) => (
                StatusCode::BAD_GATEWAY,
                format!("Request to backend service failed: {}", err),
            ),
            ApiError::InternalError => (
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