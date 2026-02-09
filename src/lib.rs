//! # Hlavi Core
//!
//! Core business logic and domain models for Hlavi kanban task management.
//!
//! This crate provides the fundamental types and operations for managing
//! kanban boards, tasks, and workflows without any dependency on
//! specific UI implementations or storage backends.

pub mod domain;
pub mod error;
pub mod storage;

// Re-export commonly used types
pub use domain::{
    board::{Board, BoardConfig, Column},
    sorting::{sort_tasks, SortField, SortOrder},
    task::{AcceptanceCriteria, Task, TaskId, TaskStatus},
};
pub use error::{HlaviError, Result};
pub use storage::Storage;
