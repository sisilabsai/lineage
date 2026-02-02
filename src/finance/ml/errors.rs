//! Error types for ML operations

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlError {
    #[error("Initialization error: {0}")]
    InitializationError(String),
    
    #[error("Tensor operation failed: {0}")]
    TensorError(String),
    
    #[error("Model serialization failed: {0}")]
    SerializationError(String),
    
    #[error("Training error: {0}")]
    TrainingError(String),
    
    #[error("Mutation error: {0}")]
    MutationError(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for ML operations
pub type Result<T> = std::result::Result<T, MlError>;
