use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

#[derive(Debug, Clone)]
pub struct VoteRecord {
    pub member_id: String,
    pub choice: VoteChoice,
    pub energy_cost: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct VoteReceipt {
    pub proposal_id: String,
    pub member_id: String,
    pub choice: VoteChoice,
    pub energy_cost: u64,
    pub timestamp: DateTime<Utc>,
}
