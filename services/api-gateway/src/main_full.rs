use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod middleware;
mod models;
mod routes;
mod service_discovery;

use crate::config::Config;
use crate::error::ApiError;
use crate::service_discovery::ServiceRegistry;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
struct AppState {
    service_registry: ServiceRegistry,
    config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting API Gateway on port {}", config.port);

    // Initialize service registry
    let service_registry = ServiceRegistry::new(&config.consul_addr).await?;

    // Create shared state
    let state = Arc::new(RwLock::new(AppState {
        service_registry,
        config: config.clone(),
    }));

    // Build application
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/services", get(list_services))
        .route("/api/v1/stake/:chain", post(proxy_stake_request))
        .route("/api/v1/balance/:chain/:address", get(proxy_balance_request))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("API Gateway listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn list_services(
    State(state): State<SharedState>,
) -> Result<Json<Vec<String>>, ApiError> {
    let app_state = state.read().await;
    let services = app_state.service_registry.list_services().await?;
    Ok(Json(services))
}

async fn proxy_stake_request(
    Path(chain): Path<String>,
    State(state): State<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app_state = state.read().await;
    
    // Find the appropriate node service
    let service_name = format!("{}-node", chain);
    let service_url = app_state
        .service_registry
        .get_service(&service_name)
        .await?
        .ok_or_else(|| ApiError::ServiceNotFound(service_name))?;
    
    // Forward request to node service
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/stake", service_url))
        .json(&payload)
        .send()
        .await?;
    
    let result = response.json().await?;
    Ok(Json(result))
}

async fn proxy_balance_request(
    Path((chain, address)): Path<(String, String)>,
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app_state = state.read().await;
    
    // Find the appropriate node service
    let service_name = format!("{}-node", chain);
    let service_url = app_state
        .service_registry
        .get_service(&service_name)
        .await?
        .ok_or_else(|| ApiError::ServiceNotFound(service_name))?;
    
    // Forward request to node service
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/balance/{}", service_url, address))
        .send()
        .await?;
    
    let result = response.json().await?;
    Ok(Json(result))
}