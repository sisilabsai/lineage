//! Integration with lineage-finance agent system

#[cfg(feature = "ml")]
pub mod adapter;

#[cfg(feature = "ml")]
pub mod agent_lifecycle;

#[cfg(feature = "ml")]
pub use adapter::{MlStrategyAdapter, create_q_net_strategy};
