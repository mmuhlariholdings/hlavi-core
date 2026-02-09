use crate::{
    domain::{Board, Task, TaskId},
    error::Result,
};
use async_trait::async_trait;

pub mod file_storage;

#[cfg(feature = "sqlite-storage")]
pub mod sqlite_storage;

/// Storage trait for persisting tasks and board state
#[async_trait]
pub trait Storage: Send + Sync {
    /// Initializes the storage backend
    async fn initialize(&self) -> Result<()>;

    /// Saves a task
    async fn save_task(&self, task: &Task) -> Result<()>;

    /// Loads a task by ID
    async fn load_task(&self, id: &TaskId) -> Result<Task>;

    /// Lists all task IDs
    async fn list_task_ids(&self) -> Result<Vec<TaskId>>;

    /// Searches for tasks matching the query in title, description, or acceptance criteria
    /// Returns a vector of tasks that match the query (case-insensitive)
    async fn search_tasks(&self, query: &str) -> Result<Vec<Task>>;

    /// Deletes a task
    async fn delete_task(&self, id: &TaskId) -> Result<()>;

    /// Saves the board state
    async fn save_board(&self, board: &Board) -> Result<()>;

    /// Loads the board state
    async fn load_board(&self) -> Result<Board>;

    /// Checks if the project is initialized
    async fn is_initialized(&self) -> bool;
}
