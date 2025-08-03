use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use crate::blockchain::BlockchainNode;
use prometheus::{Encoder, TextEncoder};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod blockchain;
mod config;
mod error;
mod health;
mod metrics;
mod service_registry;

use crate::config::Config;
use crate::error::NodeError;
use crate::health::HealthChecker;
use crate::metrics::Metrics;
use crate::service_registry::ServiceRegistrar;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
struct AppState {
    config: Config,
    health_checker: HealthChecker,
    metrics: Arc<Metrics>,
    blockchain_client: blockchain::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct StakeRequest {
    amount: String,
    address: String,
    duration_days: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StakeResponse {
    transaction_id: String,
    status: String,
    estimated_rewards: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BalanceResponse {
    address: String,
    balance: String,
    staked_amount: String,
    pending_rewards: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "node_template=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting {} on port {}", config.service_name, config.port);

    // Initialize components
    let health_checker = HealthChecker::new();
    let metrics = Arc::new(Metrics::new()?);
    let blockchain_client = blockchain::Client::new(&config)?;

    // Register with service discovery
    let registrar = ServiceRegistrar::new(&config.consul_addr)?;
    registrar
        .register(&config.service_name, &config.host, config.port)
        .await?;

    // Create shared state
    let state = Arc::new(RwLock::new(AppState {
        config: config.clone(),
        health_checker,
        metrics,
        blockchain_client,
    }));

    // Build application
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
        .route("/stake", post(stake_handler))
        .route("/balance/:address", get(balance_handler))
        .route("/info", get(info_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("{} listening on {}", config.service_name, addr);

    // Graceful shutdown
    let graceful = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }

    // Deregister from service discovery
    registrar.deregister(&config.service_name).await?;

    Ok(())
}

async fn health_check(State(state): State<SharedState>) -> Result<String, NodeError> {
    let app_state = state.read().await;
    if app_state.health_checker.is_healthy().await {
        Ok("OK".to_string())
    } else {
        Err(NodeError::ServiceUnavailable)
    }
}

async fn metrics_handler(State(_state): State<SharedState>) -> Result<String, NodeError> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

async fn stake_handler(
    State(state): State<SharedState>,
    Json(request): Json<StakeRequest>,
) -> Result<Json<StakeResponse>, NodeError> {
    let app_state = state.write().await;

    // Record metric
    app_state.metrics.stake_requests.inc();

    // Process stake request
    let result = app_state
        .blockchain_client
        .stake(&request.address, &request.amount, request.duration_days)
        .await?;

    Ok(Json(StakeResponse {
        transaction_id: result.tx_id,
        status: "pending".to_string(),
        estimated_rewards: result.estimated_rewards,
    }))
}

async fn balance_handler(
    State(state): State<SharedState>,
    axum::extract::Path(address): axum::extract::Path<String>,
) -> Result<Json<BalanceResponse>, NodeError> {
    let app_state = state.read().await;

    // Record metric
    app_state.metrics.balance_requests.inc();

    // Get balance information
    let balance_info = app_state.blockchain_client.get_balance(&address).await?;

    Ok(Json(BalanceResponse {
        address,
        balance: balance_info.balance,
        staked_amount: balance_info.staked,
        pending_rewards: balance_info.rewards,
    }))
}

async fn info_handler(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, NodeError> {
    let app_state = state.read().await;

    Ok(Json(serde_json::json!({
        "service": app_state.config.service_name,
        "version": env!("CARGO_PKG_VERSION"),
        "chain": app_state.config.chain_name,
        "status": "running"
    })))
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
