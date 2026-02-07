use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Unique identifier for a ticket (e.g., TIK001, TIK002)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TicketId(String);

impl TicketId {
    /// Creates a new TicketId from a counter
    pub fn new(counter: u32) -> Self {
        Self(format!("TIK{:03}", counter))
    }

    /// Returns the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for TicketId {
    type Err = crate::error::HlaviError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("TIK") && s.len() >= 4 {
            Ok(Self(s.to_string()))
        } else {
            Err(crate::error::HlaviError::InvalidTicketId(s.to_string()))
        }
    }
}

impl fmt::Display for TicketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Status of a ticket on the kanban board
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    New,
    Open,
    InProgress,
    Pending,
    Review,
    Done,
    Closed,
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "New"),
            Self::Open => write!(f, "Open"),
            Self::InProgress => write!(f, "In Progress"),
            Self::Pending => write!(f, "Pending"),
            Self::Review => write!(f, "Review"),
            Self::Done => write!(f, "Done"),
            Self::Closed => write!(f, "Closed"),
        }
    }
}

impl TicketStatus {
    /// Checks if a status transition is valid
    pub fn can_transition_to(&self, target: &TicketStatus) -> bool {
        match (self, target) {
            // From New
            (Self::New, Self::Open) => true,

            // From Open
            (Self::Open, Self::InProgress) => true,
            (Self::Open, Self::Closed) => true,

            // From InProgress
            (Self::InProgress, Self::Pending) => true,
            (Self::InProgress, Self::Review) => true,
            (Self::InProgress, Self::Open) => true, // Rejected back

            // From Pending
            (Self::Pending, Self::Review) => true,
            (Self::Pending, Self::InProgress) => true,

            // From Review
            (Self::Review, Self::Done) => true,
            (Self::Review, Self::InProgress) => true, // Rejected back

            // From Done
            (Self::Done, Self::Closed) => true,
            (Self::Done, Self::InProgress) => true, // Reopened

            // Same status is always valid
            _ if self == target => true,

            _ => false,
        }
    }
}

/// Acceptance criteria for a ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceCriteria {
    pub id: usize,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl AcceptanceCriteria {
    pub fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }
}

/// A kanban ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub acceptance_criteria: Vec<AcceptanceCriteria>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub agent_assigned: bool,
    pub rejection_reason: Option<String>,
}

impl Ticket {
    /// Creates a new ticket with the given ID and title
    pub fn new(id: TicketId, title: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description: None,
            status: TicketStatus::New,
            acceptance_criteria: Vec::new(),
            created_at: now,
            updated_at: now,
            agent_assigned: false,
            rejection_reason: None,
        }
    }

    /// Sets the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = Utc::now();
    }

    /// Adds an acceptance criterion
    pub fn add_acceptance_criterion(&mut self, description: String) {
        let id = self.acceptance_criteria.len() + 1;
        self.acceptance_criteria
            .push(AcceptanceCriteria::new(id, description));
        self.updated_at = Utc::now();
    }

    /// Removes an acceptance criterion by description or index
    pub fn remove_acceptance_criterion(
        &mut self,
        identifier: &str,
    ) -> Result<(), crate::error::HlaviError> {
        // Try to parse as index first
        if let Ok(index) = identifier.parse::<usize>() {
            if index > 0 && index <= self.acceptance_criteria.len() {
                self.acceptance_criteria.remove(index - 1);
                self.updated_at = Utc::now();
                return Ok(());
            }
        }

        // Try to find by description
        if let Some(pos) = self
            .acceptance_criteria
            .iter()
            .position(|ac| ac.description == identifier)
        {
            self.acceptance_criteria.remove(pos);
            self.updated_at = Utc::now();
            return Ok(());
        }

        Err(crate::error::HlaviError::AcceptanceCriteriaNotFound)
    }

    /// Changes the ticket status
    pub fn transition_to(
        &mut self,
        new_status: TicketStatus,
        rejection_reason: Option<String>,
    ) -> Result<(), crate::error::HlaviError> {
        if !self.status.can_transition_to(&new_status) {
            return Err(crate::error::HlaviError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: new_status.to_string(),
            });
        }

        self.status = new_status;
        self.rejection_reason = rejection_reason;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Checks if all acceptance criteria are completed
    pub fn all_acceptance_criteria_completed(&self) -> bool {
        !self.acceptance_criteria.is_empty()
            && self.acceptance_criteria.iter().all(|ac| ac.completed)
    }

    /// Checks if the ticket can be marked as done
    pub fn can_mark_done(&self) -> bool {
        self.status == TicketStatus::Review && self.all_acceptance_criteria_completed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket_id_creation() {
        let id = TicketId::new(1);
        assert_eq!(id.as_str(), "TIK001");

        let id = TicketId::new(42);
        assert_eq!(id.as_str(), "TIK042");
    }

    #[test]
    fn test_ticket_id_parsing() {
        let id = TicketId::from_str("TIK123").unwrap();
        assert_eq!(id.as_str(), "TIK123");

        assert!(TicketId::from_str("INVALID").is_err());
    }

    #[test]
    fn test_status_transitions() {
        assert!(TicketStatus::New.can_transition_to(&TicketStatus::Open));
        assert!(TicketStatus::Open.can_transition_to(&TicketStatus::InProgress));
        assert!(!TicketStatus::New.can_transition_to(&TicketStatus::Done));
    }

    #[test]
    fn test_ticket_acceptance_criteria() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());

        ticket.add_acceptance_criterion("AC 1".to_string());
        ticket.add_acceptance_criterion("AC 2".to_string());

        assert_eq!(ticket.acceptance_criteria.len(), 2);
        assert!(!ticket.all_acceptance_criteria_completed());
    }
}
