use chrono::{DateTime, Utc};

use crate::governance::{ProposalId, ProposalOutcome, ProposalRisk, VoteChoice};

/// Append-only governance event log.
#[derive(Debug, Default)]
pub struct GovernanceLedger {
    events: Vec<GovernanceEvent>,
}

impl GovernanceLedger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append(&mut self, event: GovernanceEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[GovernanceEvent] {
        &self.events
    }
}

/// Immutable governance events.
#[derive(Debug, Clone)]
pub enum GovernanceEvent {
    MemberAdded {
        member_id: String,
        name: String,
        energy: u64,
        timestamp: DateTime<Utc>,
    },
    ProposalCreated {
        proposal_id: ProposalId,
        title: String,
        risk: ProposalRisk,
        timestamp: DateTime<Utc>,
    },
    VoteCast {
        proposal_id: ProposalId,
        member_id: String,
        choice: VoteChoice,
        energy_cost: u64,
        timestamp: DateTime<Utc>,
    },
    ProposalClosed {
        proposal_id: ProposalId,
        outcome: ProposalOutcome,
        for_votes: u32,
        against_votes: u32,
        abstain_votes: u32,
        turnout_pct: f64,
        timestamp: DateTime<Utc>,
    },
    DissentScarred {
        proposal_id: ProposalId,
        member_id: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    ProposalExecuted {
        proposal_id: ProposalId,
        success: bool,
        error: Option<String>,
        timestamp: DateTime<Utc>,
    },
}
