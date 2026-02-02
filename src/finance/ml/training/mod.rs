//! Training infrastructure for ML models

#[cfg(feature = "ml")]
pub mod replay_buffer;

#[cfg(feature = "ml")]
pub mod optimizer;

#[cfg(feature = "ml")]
pub mod rewards;

#[cfg(feature = "ml")]
pub mod coordinator;

#[cfg(feature = "ml")]
pub mod advanced;

#[cfg(feature = "ml")]
pub mod visualization;

#[cfg(feature = "ml")]
pub use replay_buffer::ReplayBuffer;

#[cfg(feature = "ml")]
pub use optimizer::QLearningTrainer;

#[cfg(feature = "ml")]
pub use rewards::RewardCalculator;

#[cfg(feature = "ml")]
pub use coordinator::TrainingCoordinator;

#[cfg(feature = "ml")]
pub use advanced::{AdvancedTrainingCoordinator, AdvancedTrainingConfig, AdvancedQLearningTrainer};

#[cfg(feature = "ml")]
pub use visualization::{MetricsRecorder, EpisodeLog, ResultsExporter};
