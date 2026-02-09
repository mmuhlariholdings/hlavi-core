use crate::{
    domain::{Board, Task, TaskId},
    error::{HlaviError, Result},
    storage::Storage,
};
use async_trait::async_trait;

/// SQLite-based storage backend for tasks and board state
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

    async fn save_task(&self, _task: &Task) -> Result<()> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn load_task(&self, _id: &TaskId) -> Result<Task> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn list_task_ids(&self) -> Result<Vec<TaskId>> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn search_tasks(&self, _query: &str) -> Result<Vec<Task>> {
        Err(HlaviError::StorageError(
            "SQLite storage not yet implemented".to_string(),
        ))
    }

    async fn delete_task(&self, _id: &TaskId) -> Result<()> {
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
