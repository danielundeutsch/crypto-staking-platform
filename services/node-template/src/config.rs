use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub service_name: String,
    pub chain_name: String,
    pub host: String,
    pub port: u16,
    pub consul_addr: String,
    pub node_rpc_url: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            service_name: env::var("SERVICE_NAME")?,
            chain_name: env::var("CHAIN_NAME")?,
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            consul_addr: env::var("CONSUL_ADDR")
                .unwrap_or_else(|_| "http://localhost:8500".to_string()),
            node_rpc_url: env::var("NODE_RPC_URL")?,
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}