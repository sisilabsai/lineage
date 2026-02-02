//! Neural network models for trading

#[cfg(feature = "ml")]
pub mod base;

#[cfg(feature = "ml")]
pub mod q_net;

#[cfg(feature = "ml")]
pub use q_net::SimpleQNet;
