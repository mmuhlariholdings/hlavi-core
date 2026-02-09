use crate::{
    domain::{Board, Ticket, TicketId},
    error::Result,
};
use async_trait::async_trait;

pub mod file_storage;

#[cfg(feature = "sqlite-storage")]
pub mod sqlite_storage;

/// Storage trait for persisting tickets and board state
#[async_trait]
pub trait Storage: Send + Sync {
    /// Initializes the storage backend
    async fn initialize(&self) -> Result<()>;

    /// Saves a ticket
    async fn save_ticket(&self, ticket: &Ticket) -> Result<()>;

    /// Loads a ticket by ID
    async fn load_ticket(&self, id: &TicketId) -> Result<Ticket>;

    /// Lists all ticket IDs
    async fn list_ticket_ids(&self) -> Result<Vec<TicketId>>;

    /// Searches for tickets matching the query in title, description, or acceptance criteria
    /// Returns a vector of tickets that match the query (case-insensitive)
    async fn search_tickets(&self, query: &str) -> Result<Vec<Ticket>>;

    /// Deletes a ticket
    async fn delete_ticket(&self, id: &TicketId) -> Result<()>;

    /// Saves the board state
    async fn save_board(&self, board: &Board) -> Result<()>;

    /// Loads the board state
    async fn load_board(&self) -> Result<Board>;

    /// Checks if the project is initialized
    async fn is_initialized(&self) -> bool;
}
