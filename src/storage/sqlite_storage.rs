use crate::{
    domain::{Board, Ticket, TicketId},
    error::{HlaviError, Result},
    storage::Storage,
};
use async_trait::async_trait;

/// SQLite-based storage backend for tickets and board state
pub struct SqliteStorage {
    _connection: (), // Placeholder for future implementation
}

impl SqliteStorage {
    /// Creates a new SQLite storage instance
    pub fn new(_database_path: &str) -> Result<Self> {
        // TODO: Implement SQLite storage
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn initialize(&self) -> Result<()> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn save_ticket(&self, _ticket: &Ticket) -> Result<()> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn load_ticket(&self, _id: &TicketId) -> Result<Ticket> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn list_ticket_ids(&self) -> Result<Vec<TicketId>> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn delete_ticket(&self, _id: &TicketId) -> Result<()> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn save_board(&self, _board: &Board) -> Result<()> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn load_board(&self) -> Result<Board> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn is_initialized(&self) -> bool {
        false
    }
}
