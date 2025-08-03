use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Starting API Gateway on port 8080");

    // Build application
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/services", get(list_services))
        .route("/api/v1/stake/:chain", post(stake_handler))
        .route("/api/v1/balance/:chain/:address", get(balance_handler))
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("API Gateway listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn list_services() -> Json<Vec<String>> {
    Json(vec![
        "api-gateway".to_string(),
        "ethereum-node".to_string(),
        "bitcoin-node".to_string(),
        "solana-node".to_string(),
    ])
}

async fn stake_handler(Path(chain): Path<String>, Json(payload): Json<Value>) -> Json<Value> {
    println!(
        "Stake request for chain: {chain} with payload: {payload:?}"
    );

    Json(json!({
        "transaction_id": format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", "")),
        "status": "pending",
        "estimated_rewards": "0.05",
        "chain": chain
    }))
}

async fn balance_handler(Path((chain, address)): Path<(String, String)>) -> Json<Value> {
    println!("Balance request for chain: {chain} address: {address}");

    Json(json!({
        "address": address,
        "balance": "1000.000000",
        "staked_amount": "500.000000",
        "pending_rewards": "25.123456",
        "chain": chain
    }))
}
