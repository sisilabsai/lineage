//! Governance module: irreversible consensus with permanent consequences.

mod council;
mod ledger;
mod proposal;
mod vote;

pub use council::{ExecutionContext, ExecutionResult, GovernanceConfig, GovernanceCouncil, GovernanceError};
pub use ledger::{GovernanceEvent, GovernanceLedger};
pub use proposal::{Proposal, ProposalId, ProposalOutcome, ProposalRisk, ProposalStatus};
pub use vote::{VoteChoice, VoteRecord, VoteReceipt};
