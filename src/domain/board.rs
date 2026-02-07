use crate::domain::ticket::{TicketId, TicketStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a kanban board column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub status: TicketStatus,
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
    pub fn new(name: String, status: TicketStatus) -> Self {
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
                Column::new("New".to_string(), TicketStatus::New),
                Column::new("Open".to_string(), TicketStatus::Open),
                Column::new("In Progress".to_string(), TicketStatus::InProgress)
                    .with_agent(AgentMode::Unattended),
                Column::new("Pending".to_string(), TicketStatus::Pending),
                Column::new("Review".to_string(), TicketStatus::Review),
                Column::new("Done".to_string(), TicketStatus::Done),
                Column::new("Closed".to_string(), TicketStatus::Closed),
            ],
        }
    }
}

/// Kanban board state
#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub config: BoardConfig,
    pub tickets: HashMap<String, TicketId>,
    pub next_ticket_number: u32,
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        Self {
            config,
            tickets: HashMap::new(),
            next_ticket_number: 1,
        }
    }

    /// Generates the next ticket ID
    pub fn next_ticket_id(&mut self) -> TicketId {
        let id = TicketId::new(self.next_ticket_number);
        self.next_ticket_number += 1;
        id
    }

    /// Adds a ticket to the board tracking
    pub fn add_ticket(&mut self, ticket_id: TicketId) {
        self.tickets
            .insert(ticket_id.as_str().to_string(), ticket_id);
    }

    /// Gets the column configuration for a status
    pub fn get_column_for_status(&self, status: &TicketStatus) -> Option<&Column> {
        self.config.columns.iter().find(|col| &col.status == status)
    }

    /// Checks if agent mode is enabled for a status
    pub fn is_agent_enabled_for_status(&self, status: &TicketStatus) -> bool {
        self.get_column_for_status(status)
            .map(|col| col.agent_enabled)
            .unwrap_or(false)
    }

    /// Gets the agent mode for a status
    pub fn get_agent_mode_for_status(&self, status: &TicketStatus) -> Option<AgentMode> {
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
        assert_eq!(board.next_ticket_number, 1);
        assert_eq!(board.tickets.len(), 0);
    }

    #[test]
    fn test_next_ticket_id() {
        let mut board = Board::default();

        let id1 = board.next_ticket_id();
        assert_eq!(id1.as_str(), "HLA1");

        let id2 = board.next_ticket_id();
        assert_eq!(id2.as_str(), "HLA2");
    }

    #[test]
    fn test_agent_configuration() {
        let board = Board::default();

        assert!(board.is_agent_enabled_for_status(&TicketStatus::InProgress));
        assert!(!board.is_agent_enabled_for_status(&TicketStatus::New));

        let mode = board.get_agent_mode_for_status(&TicketStatus::InProgress);
        assert_eq!(mode, Some(AgentMode::Unattended));
    }
}
