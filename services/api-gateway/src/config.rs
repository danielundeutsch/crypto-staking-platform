use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub consul_addr: String,
    pub service_name: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            consul_addr: env::var("CONSUL_ADDR")
                .unwrap_or_else(|_| "http://localhost:8500".to_string()),
            service_name: env::var("SERVICE_NAME")
                .unwrap_or_else(|_| "api-gateway".to_string()),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}