//! # Lineage - Software Identity Through Irreversible Change
//!
//! Lineage is an ontological system for software that experiences consequences.
//!
//! ## Core Guarantees
//!
//! - **Identity cannot be cloned or copied**
//! - **History cannot be erased or rewritten**
//! - **Energy cannot be restored or recharged**
//! - **Scars are permanent and visible forever**
//! - **Death is final and irreversible**
//!
//! ## Quick Start
//!
//! ```rust
//! use lineage::{Lineage, OperationResult};
//! use lineage::scar::ScarSeverity;
//!
//! // Create a lineage with 1000 energy units
//! let mut lineage = Lineage::create(1000);
//!
//! // Perform an operation (costs energy)
//! match lineage.perform_operation("Initialize".to_string(), 100) {
//!     OperationResult::Success { energy_consumed } => {
//!         println!("Success! Used {} energy", energy_consumed);
//!     }
//!     OperationResult::InsufficientEnergy { required, available } => {
//!         println!("Failed: need {}, have {}", required, available);
//!     }
//!     OperationResult::Dead => {
//!         println!("Lineage is dead");
//!     }
//!     OperationResult::OntologicalViolation { reason } => {
//!         eprintln!("FATAL: {}", reason);
//!     }
//! }
//!
//! // Check status
//! println!("{}", lineage.status());
//! ```
//!
//! ## Ontological Principles
//!
//! This system enforces **correctness over convenience**:
//!
//! 1. **No rollback** - History is append-only and immutable
//! 2. **No resurrection** - Death is permanent
//! 3. **No duplication** - Identity cannot be cloned
//! 4. **No recharge** - Energy is finite and non-renewable
//! 5. **No healing** - Scars are permanent
//!
//! These are not limitations. These are **fundamental design principles**.
//!
//! ## Architecture
//!
//! - [`identity`] - Unique, non-copyable identity system
//! - [`memory`] - Append-only causal event log
//! - [`metabolism`] - Finite energy and death
//! - [`scar`] - Permanent error recording
//! - [`lineage`] - Complete lifecycle orchestration

pub mod identity;
pub mod memory;
pub mod metabolism;
pub mod scar;
pub mod lineage;
pub mod behavior;
pub mod trust;
pub mod agent;

// Re-export main types for convenience
pub use lineage::{Lineage, OperationResult, OperationError, LineageStatus};
pub use identity::Identity;
pub use memory::{Memory, Event};
pub use metabolism::{Metabolism, ConsumptionResult};
pub use scar::{ScarTissue, Scar, ScarSeverity};
pub use behavior::{PulseBehavior, PulseOutput};
pub use trust::{TrustedActor, TrustCapability, TrustResult, ViolationType};
pub use agent::{TaskAgent, Task, TaskOutcome, TaskResult};
