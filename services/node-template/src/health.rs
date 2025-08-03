use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct HealthChecker {
    last_check: Arc<RwLock<Instant>>,
    is_healthy: Arc<RwLock<bool>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        HealthChecker {
            last_check: Arc::new(RwLock::new(Instant::now())),
            is_healthy: Arc::new(RwLock::new(true)),
        }
    }

    pub async fn is_healthy(&self) -> bool {
        let is_healthy = self.is_healthy.read().await;
        *is_healthy
    }

    pub async fn set_healthy(&self, healthy: bool) {
        let mut is_healthy = self.is_healthy.write().await;
        let mut last_check = self.last_check.write().await;
        *is_healthy = healthy;
        *last_check = Instant::now();
    }

    pub async fn check_blockchain_connection(&self) -> bool {
        // This would check actual blockchain connectivity
        // For now, we'll simulate it
        true
    }

    pub async fn perform_health_check(&self) -> bool {
        let blockchain_ok = self.check_blockchain_connection().await;
        self.set_healthy(blockchain_ok).await;
        blockchain_ok
    }
}