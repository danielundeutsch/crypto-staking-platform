use prometheus::{IntCounter, Registry, Histogram, HistogramOpts};
use crate::error::NodeError;

pub struct Metrics {
    pub stake_requests: IntCounter,
    pub balance_requests: IntCounter,
    pub blockchain_latency: Histogram,
    pub successful_stakes: IntCounter,
    pub failed_stakes: IntCounter,
}

impl Metrics {
    pub fn new() -> Result<Self, NodeError> {
        let stake_requests = IntCounter::new("stake_requests_total", "Total number of stake requests")?;
        let balance_requests = IntCounter::new("balance_requests_total", "Total number of balance requests")?;
        let successful_stakes = IntCounter::new("successful_stakes_total", "Total number of successful stakes")?;
        let failed_stakes = IntCounter::new("failed_stakes_total", "Total number of failed stakes")?;
        
        let blockchain_latency = Histogram::with_opts(
            HistogramOpts::new("blockchain_latency_seconds", "Blockchain RPC call latency")
                .buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0])
        )?;

        // Register metrics
        let registry = Registry::new();
        registry.register(Box::new(stake_requests.clone()))?;
        registry.register(Box::new(balance_requests.clone()))?;
        registry.register(Box::new(successful_stakes.clone()))?;
        registry.register(Box::new(failed_stakes.clone()))?;
        registry.register(Box::new(blockchain_latency.clone()))?;

        Ok(Metrics {
            stake_requests,
            balance_requests,
            blockchain_latency,
            successful_stakes,
            failed_stakes,
        })
    }
}