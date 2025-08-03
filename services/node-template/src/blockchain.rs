use crate::config::Config;
use crate::error::NodeError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeResult {
    pub tx_id: String,
    pub estimated_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub balance: String,
    pub staked: String,
    pub rewards: String,
}

#[async_trait]
pub trait BlockchainNode {
    async fn stake(
        &self,
        address: &str,
        amount: &str,
        duration_days: Option<u32>,
    ) -> Result<StakeResult, NodeError>;
    async fn get_balance(&self, address: &str) -> Result<BalanceInfo, NodeError>;
    async fn validate_address(&self, address: &str) -> Result<bool, NodeError>;
    async fn get_staking_info(&self) -> Result<StakingInfo, NodeError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingInfo {
    pub min_stake_amount: String,
    pub max_stake_amount: String,
    pub current_apr: f64,
    pub lock_periods: Vec<u32>,
}

#[derive(Clone)]
pub struct Client {
    rpc_url: String,
    chain_name: String,
}

impl Client {
    pub fn new(config: &Config) -> Result<Self, NodeError> {
        Ok(Client {
            rpc_url: config.node_rpc_url.clone(),
            chain_name: config.chain_name.clone(),
        })
    }
}

#[async_trait]
impl BlockchainNode for Client {
    async fn stake(
        &self,
        address: &str,
        amount: &str,
        duration_days: Option<u32>,
    ) -> Result<StakeResult, NodeError> {
        // Validate address first
        if !self.validate_address(address).await? {
            return Err(NodeError::InvalidAddress(address.to_string()));
        }

        // In a real implementation, this would interact with the blockchain
        // For now, we'll simulate the staking operation
        let tx_id = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        let apr = 5.0; // Example APR
        let days = duration_days.unwrap_or(30);

        // Simple reward calculation
        let amount_f64: f64 = amount
            .parse()
            .map_err(|_| NodeError::StakingError("Invalid amount".to_string()))?;
        let estimated_rewards = amount_f64 * (apr / 100.0) * (days as f64 / 365.0);

        Ok(StakeResult {
            tx_id,
            estimated_rewards: format!("{estimated_rewards:.6}"),
        })
    }

    async fn get_balance(&self, address: &str) -> Result<BalanceInfo, NodeError> {
        // Validate address first
        if !self.validate_address(address).await? {
            return Err(NodeError::InvalidAddress(address.to_string()));
        }

        // In a real implementation, this would query the blockchain
        // For now, we'll return mock data
        Ok(BalanceInfo {
            balance: "1000.000000".to_string(),
            staked: "500.000000".to_string(),
            rewards: "25.123456".to_string(),
        })
    }

    async fn validate_address(&self, address: &str) -> Result<bool, NodeError> {
        // Basic validation - in reality this would be chain-specific
        match self.chain_name.as_str() {
            "ethereum" => Ok(address.starts_with("0x") && address.len() == 42),
            "bitcoin" => Ok((address.starts_with("1")
                || address.starts_with("3")
                || address.starts_with("bc1"))
                && address.len() >= 26),
            "solana" => Ok(address.len() == 44), // Base58 encoded
            _ => Ok(address.len() > 10),         // Generic validation
        }
    }

    async fn get_staking_info(&self) -> Result<StakingInfo, NodeError> {
        // This would fetch real staking parameters from the blockchain
        Ok(StakingInfo {
            min_stake_amount: "10.0".to_string(),
            max_stake_amount: "1000000.0".to_string(),
            current_apr: 5.0,
            lock_periods: vec![7, 14, 30, 90, 180, 365],
        })
    }
}
