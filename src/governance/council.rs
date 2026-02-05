use std::collections::HashMap;

use chrono::Utc;

use crate::agent::{Task, TaskAgent, TaskOutcome, TaskResult};
use crate::governance::ledger::{GovernanceEvent, GovernanceLedger};
use crate::governance::proposal::{Proposal, ProposalId, ProposalOutcome, ProposalRisk, ProposalStatus};
use crate::governance::vote::{VoteChoice, VoteRecord, VoteReceipt};

#[derive(Debug, Clone)]
pub struct GovernanceConfig {
    pub vote_cost: u64,
    pub abstain_cost: u64,
    pub quorum_pct: f64,
    pub pass_pct: f64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            vote_cost: 25,
            abstain_cost: 10,
            quorum_pct: 50.0,
            pass_pct: 60.0,
        }
    }
}

pub struct GovernanceCouncil {
    config: GovernanceConfig,
    members: HashMap<String, CouncilMember>,
    proposals: HashMap<ProposalId, Proposal>,
    ledger: GovernanceLedger,
}

struct CouncilMember {
    name: String,
    agent: TaskAgent,
    buried: bool,
}

impl GovernanceCouncil {
    pub fn new(config: GovernanceConfig) -> Self {
        Self {
            config,
            members: HashMap::new(),
            proposals: HashMap::new(),
            ledger: GovernanceLedger::new(),
        }
    }

    pub fn config(&self) -> &GovernanceConfig {
        &self.config
    }

    pub fn ledger(&self) -> &GovernanceLedger {
        &self.ledger
    }

    pub fn member_ids(&self) -> Vec<String> {
        self.members.keys().cloned().collect()
    }

    pub fn member_name(&self, member_id: &str) -> Option<&str> {
        self.members.get(member_id).map(|m| m.name.as_str())
    }

    pub fn member_energy(&self, member_id: &str) -> Option<u64> {
        self.members.get(member_id).map(|m| m.agent.energy())
    }

    pub fn member_damage(&self, member_id: &str) -> Option<u32> {
        self.members.get(member_id).map(|m| m.agent.damage_score())
    }

    pub fn member_is_alive(&self, member_id: &str) -> Option<bool> {
        self.members.get(member_id).map(|m| m.agent.is_alive())
    }

    pub fn add_member(&mut self, name: String, initial_energy: u64) -> String {
        let agent = TaskAgent::create(initial_energy);
        let member_id = agent.identity().id().to_string();
        let member = CouncilMember {
            name: name.clone(),
            agent,
            buried: false,
        };

        self.members.insert(member_id.clone(), member);
        self.ledger.append(GovernanceEvent::MemberAdded {
            member_id: member_id.clone(),
            name,
            energy: initial_energy,
            timestamp: Utc::now(),
        });

        member_id
    }

    /// Bury any dead members in the graveyard.
    /// Returns the IDs of members newly buried.
    pub fn bury_dead_members(&mut self) -> Vec<String> {
        let mut buried = Vec::new();

        for (member_id, member) in self.members.iter_mut() {
            if member.buried {
                continue;
            }

            if member.agent.is_alive() {
                continue;
            }

            if member.agent.bury().is_ok() {
                member.buried = true;
                buried.push(member_id.clone());
            }
        }

        buried
    }

    pub fn propose(
        &mut self,
        title: String,
        risk: ProposalRisk,
        voting_window_secs: i64,
    ) -> ProposalId {
        let proposal = Proposal::new(title.clone(), risk, voting_window_secs);
        let proposal_id = proposal.id().clone();

        self.proposals.insert(proposal_id.clone(), proposal);
        self.ledger.append(GovernanceEvent::ProposalCreated {
            proposal_id: proposal_id.clone(),
            title,
            risk,
            timestamp: Utc::now(),
        });

        proposal_id
    }

    pub fn vote(
        &mut self,
        proposal_id: ProposalId,
        member_id: &str,
        choice: VoteChoice,
    ) -> Result<VoteReceipt, GovernanceError> {
        let (status, already_voted, title) = {
            let proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(GovernanceError::ProposalNotFound)?;
            (
                proposal.status(),
                proposal.votes().iter().any(|vote| vote.member_id == member_id),
                proposal.title().to_string(),
            )
        };

        if status != ProposalStatus::Open {
            return Err(GovernanceError::ProposalClosed);
        }

        if already_voted {
            return Err(GovernanceError::AlreadyVoted);
        }

        let energy_cost = match choice {
            VoteChoice::Abstain => self.config.abstain_cost,
            _ => self.config.vote_cost,
        };

        let task = Task::new(format!("Governance vote: {}", title), energy_cost);
        {
            let member = self
                .members
                .get_mut(member_id)
                .ok_or(GovernanceError::MemberNotFound)?;

            match member.agent.execute_task(task, TaskOutcome::Success) {
                TaskResult::Completed { .. } => {}
                TaskResult::InsufficientEnergy { required, available } => {
                    return Err(GovernanceError::InsufficientEnergy { required, available });
                }
                TaskResult::AgentTerminated => {
                    return Err(GovernanceError::MemberTerminated);
                }
                TaskResult::CapacityInsufficient { reason } => {
                    return Err(GovernanceError::CapacityInsufficient(reason));
                }
                TaskResult::Failed { reason, .. } => {
                    return Err(GovernanceError::VoteFailed(reason));
                }
            }
        }

        let timestamp = Utc::now();
        let record = VoteRecord {
            member_id: member_id.to_string(),
            choice,
            energy_cost,
            timestamp,
        };

        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            proposal.record_vote(record.clone());
        }

        self.ledger.append(GovernanceEvent::VoteCast {
            proposal_id: proposal_id.clone(),
            member_id: member_id.to_string(),
            choice,
            energy_cost,
            timestamp,
        });

        Ok(VoteReceipt {
            proposal_id: proposal_id.as_str().to_string(),
            member_id: member_id.to_string(),
            choice,
            energy_cost,
            timestamp,
        })
    }

    pub fn vote_by_id(
        &mut self,
        proposal_id: &str,
        member_id: &str,
        choice: VoteChoice,
    ) -> Result<VoteReceipt, GovernanceError> {
        let mut matches = self
            .proposals
            .keys()
            .filter(|id| id.as_str() == proposal_id || id.as_str().starts_with(proposal_id))
            .cloned();

        let resolved = match matches.next() {
            Some(id) => id,
            None => return Err(GovernanceError::ProposalNotFound),
        };

        if matches.next().is_some() {
            return Err(GovernanceError::ProposalAmbiguous);
        }

        self.vote(resolved, member_id, choice)
    }

    pub fn close(&mut self, proposal_id: ProposalId) -> Result<ProposalOutcome, GovernanceError> {
        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;

        if proposal.status() != ProposalStatus::Open {
            return Err(GovernanceError::ProposalClosed);
        }

        let mut for_votes = 0u32;
        let mut against_votes = 0u32;
        let mut abstain_votes = 0u32;

        for vote in proposal.votes() {
            match vote.choice {
                VoteChoice::For => for_votes += 1,
                VoteChoice::Against => against_votes += 1,
                VoteChoice::Abstain => abstain_votes += 1,
            }
        }

        let total_votes = for_votes + against_votes + abstain_votes;
        let total_members = self.members.len() as f64;
        let turnout_pct = if total_members == 0.0 {
            0.0
        } else {
            (total_votes as f64 / total_members) * 100.0
        };

        let outcome = if turnout_pct < self.config.quorum_pct {
            ProposalOutcome::NoQuorum
        } else {
            let decided_votes = for_votes + against_votes;
            if decided_votes == 0 {
                ProposalOutcome::Failed
            } else {
                let for_pct = (for_votes as f64 / decided_votes as f64) * 100.0;
                if for_pct >= self.config.pass_pct {
                    ProposalOutcome::Passed
                } else {
                    ProposalOutcome::Failed
                }
            }
        };

        proposal.set_outcome(outcome);
        let risk = proposal.risk();
        let votes_snapshot = proposal.votes().to_vec();

        self.ledger.append(GovernanceEvent::ProposalClosed {
            proposal_id: proposal_id.clone(),
            outcome,
            for_votes,
            against_votes,
            abstain_votes,
            turnout_pct,
            timestamp: Utc::now(),
        });

        self.apply_dissent_penalties(proposal_id.clone(), risk, &votes_snapshot, outcome)?;

        Ok(outcome)
    }

    fn apply_dissent_penalties(
        &mut self,
        proposal_id: ProposalId,
        risk: ProposalRisk,
        votes: &[VoteRecord],
        outcome: ProposalOutcome,
    ) -> Result<(), GovernanceError> {
        let dissenters: Vec<String> = match outcome {
            ProposalOutcome::Passed => votes
                .iter()
                .filter(|vote| matches!(vote.choice, VoteChoice::Against))
                .map(|vote| vote.member_id.clone())
                .collect(),
            ProposalOutcome::Failed => votes
                .iter()
                .filter(|vote| matches!(vote.choice, VoteChoice::For))
                .map(|vote| vote.member_id.clone())
                .collect(),
            ProposalOutcome::NoQuorum => Vec::new(),
        };

        if dissenters.is_empty() {
            return Ok(());
        }

        let penalty_outcome = dissent_outcome(risk);

        for member_id in dissenters {
            if let Some(member) = self.members.get_mut(&member_id) {
                let task = Task::new("Dissent penalty".to_string(), 0);
                let result = member.agent.execute_task(task, penalty_outcome.clone());

                if matches!(result, TaskResult::Failed { .. }) {
                    self.ledger.append(GovernanceEvent::DissentScarred {
                        proposal_id: proposal_id.clone(),
                        member_id: member_id.clone(),
                        reason: "Dissent penalty applied".to_string(),
                        timestamp: Utc::now(),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn execute<F>(
        &mut self,
        proposal_id: ProposalId,
        effect: F,
    ) -> Result<ExecutionResult, GovernanceError>
    where
        F: FnOnce(ExecutionContext) -> Result<(), String>,
    {
        let (status, outcome, title, risk) = {
            let proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(GovernanceError::ProposalNotFound)?;
            (
                proposal.status(),
                proposal.outcome(),
                proposal.title().to_string(),
                proposal.risk(),
            )
        };

        if status == ProposalStatus::Executed {
            return Err(GovernanceError::AlreadyExecuted);
        }

        if outcome != Some(ProposalOutcome::Passed) {
            return Err(GovernanceError::NotExecutable);
        }

        let context = ExecutionContext {
            proposal_id: proposal_id.as_str().to_string(),
            title,
            risk,
        };

        let result = effect(context);

        let execution_result = match result {
            Ok(()) => ExecutionResult {
                success: true,
                error: None,
            },
            Err(err) => ExecutionResult {
                success: false,
                error: Some(err),
            },
        };

        self.ledger.append(GovernanceEvent::ProposalExecuted {
            proposal_id: proposal_id.clone(),
            success: execution_result.success,
            error: execution_result.error.clone(),
            timestamp: Utc::now(),
        });

        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            proposal.mark_executed();
        }

        Ok(execution_result)
    }
}

fn dissent_outcome(risk: ProposalRisk) -> TaskOutcome {
    match risk {
        ProposalRisk::Low => TaskOutcome::RecoverableFailure {
            reason: "Dissent on low-risk proposal".to_string(),
        },
        ProposalRisk::Medium => TaskOutcome::SignificantFailure {
            reason: "Dissent on medium-risk proposal".to_string(),
        },
        ProposalRisk::High => TaskOutcome::SevereFailure {
            reason: "Dissent on high-risk proposal".to_string(),
        },
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub proposal_id: String,
    pub title: String,
    pub risk: ProposalRisk,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum GovernanceError {
    MemberNotFound,
    ProposalNotFound,
    ProposalAmbiguous,
    ProposalClosed,
    AlreadyVoted,
    InsufficientEnergy { required: u64, available: u64 },
    MemberTerminated,
    CapacityInsufficient(String),
    VoteFailed(String),
    NotExecutable,
    AlreadyExecuted,
}

impl std::fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GovernanceError::MemberNotFound => write!(f, "Member not found"),
            GovernanceError::ProposalNotFound => write!(f, "Proposal not found"),
            GovernanceError::ProposalAmbiguous => {
                write!(f, "Proposal id is ambiguous, provide a longer id")
            }
            GovernanceError::ProposalClosed => write!(f, "Proposal is closed"),
            GovernanceError::AlreadyVoted => write!(f, "Member already voted"),
            GovernanceError::InsufficientEnergy { required, available } => write!(
                f,
                "Insufficient energy: required {}, available {}",
                required, available
            ),
            GovernanceError::MemberTerminated => write!(f, "Member is terminated"),
            GovernanceError::CapacityInsufficient(reason) => write!(f, "{}", reason),
            GovernanceError::VoteFailed(reason) => write!(f, "Vote failed: {}", reason),
            GovernanceError::NotExecutable => write!(f, "Proposal is not executable"),
            GovernanceError::AlreadyExecuted => write!(f, "Proposal already executed"),
        }
    }
}

impl std::error::Error for GovernanceError {}
