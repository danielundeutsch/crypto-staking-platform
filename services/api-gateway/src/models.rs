use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRequest {
    pub amount: String,
    pub address: String,
    pub duration_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeResponse {
    pub transaction_id: String,
    pub status: String,
    pub estimated_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: String,
    pub staked_amount: String,
    pub pending_rewards: String,
}