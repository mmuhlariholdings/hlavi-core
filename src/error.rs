use thiserror::Error;

pub type Result<T> = std::result::Result<T, HlaviError>;

#[derive(Debug, Error)]
pub enum HlaviError {
    #[error("Ticket not found: {0}")]
    TicketNotFound(String),

    #[error("Board not initialized")]
    BoardNotInitialized,

    #[error("Invalid ticket status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Invalid ticket ID format: {0}")]
    InvalidTicketId(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Acceptance criteria not found")]
    AcceptanceCriteriaNotFound,

    #[error("Project not initialized. Run 'hlavi init' first.")]
    ProjectNotInitialized,

    #[error("{0}")]
    Other(String),
}
