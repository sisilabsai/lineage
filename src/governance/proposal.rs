use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::governance::vote::VoteRecord;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProposalId(String);

impl ProposalId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalRisk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalStatus {
    Open,
    Closed,
    Executed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalOutcome {
    Passed,
    Failed,
    NoQuorum,
}

#[derive(Debug, Clone)]
pub struct Proposal {
    id: ProposalId,
    title: String,
    risk: ProposalRisk,
    status: ProposalStatus,
    outcome: Option<ProposalOutcome>,
    created_at: DateTime<Utc>,
    closes_at: DateTime<Utc>,
    votes: Vec<VoteRecord>,
}

impl Proposal {
    pub fn new(title: String, risk: ProposalRisk, voting_window_secs: i64) -> Self {
        let now = Utc::now();
        let closes_at = now + chrono::Duration::seconds(voting_window_secs);

        Self {
            id: ProposalId::new(),
            title,
            risk,
            status: ProposalStatus::Open,
            outcome: None,
            created_at: now,
            closes_at,
            votes: Vec::new(),
        }
    }

    pub fn id(&self) -> &ProposalId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn risk(&self) -> ProposalRisk {
        self.risk
    }

    pub fn status(&self) -> ProposalStatus {
        self.status
    }

    pub fn outcome(&self) -> Option<ProposalOutcome> {
        self.outcome
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn closes_at(&self) -> DateTime<Utc> {
        self.closes_at
    }

    pub fn votes(&self) -> &[VoteRecord] {
        &self.votes
    }

    pub fn record_vote(&mut self, vote: VoteRecord) {
        self.votes.push(vote);
    }

    pub fn set_outcome(&mut self, outcome: ProposalOutcome) {
        self.outcome = Some(outcome);
        self.status = ProposalStatus::Closed;
    }

    pub fn mark_executed(&mut self) {
        self.status = ProposalStatus::Executed;
    }
}
