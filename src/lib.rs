//! # Hlavi Core
//!
//! Core business logic and domain models for Hlavi kanban task management.
//!
//! This crate provides the fundamental types and operations for managing
//! kanban boards, tickets, and workflows without any dependency on
//! specific UI implementations or storage backends.

pub mod domain;
pub mod error;
pub mod storage;

// Re-export commonly used types
pub use domain::{
    board::{Board, BoardConfig, Column},
    sorting::{sort_tickets, SortField, SortOrder},
    ticket::{AcceptanceCriteria, Ticket, TicketId, TicketStatus},
};
pub use error::{HlaviError, Result};
pub use storage::Storage;
