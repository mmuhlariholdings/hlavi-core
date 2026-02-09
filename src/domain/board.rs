use crate::domain::task::{TaskId, TaskStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a kanban board column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub status: TaskStatus,
    pub agent_enabled: bool,
    pub agent_mode: Option<AgentMode>,
}

/// Agent execution mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Attended,
    Unattended,
}

impl Column {
    pub fn new(name: String, status: TaskStatus) -> Self {
        Self {
            name,
            status,
            agent_enabled: false,
            agent_mode: None,
        }
    }

    pub fn with_agent(mut self, mode: AgentMode) -> Self {
        self.agent_enabled = true;
        self.agent_mode = Some(mode);
        self
    }
}

/// Board configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardConfig {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Default for BoardConfig {
    fn default() -> Self {
        Self {
            name: "Default Board".to_string(),
            columns: vec![
                Column::new("New".to_string(), TaskStatus::New),
                Column::new("Open".to_string(), TaskStatus::Open),
                Column::new("In Progress".to_string(), TaskStatus::InProgress)
                    .with_agent(AgentMode::Unattended),
                Column::new("Pending".to_string(), TaskStatus::Pending),
                Column::new("Review".to_string(), TaskStatus::Review),
                Column::new("Done".to_string(), TaskStatus::Done),
                Column::new("Closed".to_string(), TaskStatus::Closed),
            ],
        }
    }
}

/// Kanban board state
#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub config: BoardConfig,
    pub tasks: HashMap<String, TaskId>,
    pub next_task_number: u32,
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        Self {
            config,
            tasks: HashMap::new(),
            next_task_number: 1,
        }
    }

    /// Generates the next task ID
    pub fn next_task_id(&mut self) -> TaskId {
        let id = TaskId::new(self.next_task_number);
        self.next_task_number += 1;
        id
    }

    /// Adds a task to the board tracking
    pub fn add_task(&mut self, task_id: TaskId) {
        self.tasks.insert(task_id.as_str().to_string(), task_id);
    }

    /// Gets the column configuration for a status
    pub fn get_column_for_status(&self, status: &TaskStatus) -> Option<&Column> {
        self.config.columns.iter().find(|col| &col.status == status)
    }

    /// Checks if agent mode is enabled for a status
    pub fn is_agent_enabled_for_status(&self, status: &TaskStatus) -> bool {
        self.get_column_for_status(status)
            .map(|col| col.agent_enabled)
            .unwrap_or(false)
    }

    /// Gets the agent mode for a status
    pub fn get_agent_mode_for_status(&self, status: &TaskStatus) -> Option<AgentMode> {
        self.get_column_for_status(status)
            .and_then(|col| col.agent_mode.clone())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(BoardConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::default();
        assert_eq!(board.next_task_number, 1);
        assert_eq!(board.tasks.len(), 0);
    }

    #[test]
    fn test_next_task_id() {
        let mut board = Board::default();

        let id1 = board.next_task_id();
        assert_eq!(id1.as_str(), "HLA1");

        let id2 = board.next_task_id();
        assert_eq!(id2.as_str(), "HLA2");
    }

    #[test]
    fn test_agent_configuration() {
        let board = Board::default();

        assert!(board.is_agent_enabled_for_status(&TaskStatus::InProgress));
        assert!(!board.is_agent_enabled_for_status(&TaskStatus::New));

        let mode = board.get_agent_mode_for_status(&TaskStatus::InProgress);
        assert_eq!(mode, Some(AgentMode::Unattended));
    }
}
