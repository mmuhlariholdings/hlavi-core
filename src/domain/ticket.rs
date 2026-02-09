use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Unique identifier for a ticket (e.g., HLA1, HLA2, HLA100)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TicketId(String);

impl TicketId {
    /// Creates a new TicketId from a counter
    pub fn new(counter: u32) -> Self {
        Self(format!("HLA{}", counter))
    }

    /// Returns the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for TicketId {
    type Err = crate::error::HlaviError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("HLA") && s.len() >= 4 {
            // Verify the rest is a valid number
            if s[3..].parse::<u32>().is_ok() {
                Ok(Self(s.to_string()))
            } else {
                Err(crate::error::HlaviError::InvalidTicketId(s.to_string()))
            }
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

    pub fn mark_incomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }

    pub fn toggle(&mut self) {
        if self.completed {
            self.mark_incomplete();
        } else {
            self.mark_completed();
        }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
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
            start_date: None,
            end_date: None,
        }
    }

    /// Sets the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = Utc::now();
    }

    /// Sets the start date with validation against end_date
    pub fn set_start_date(&mut self, date: DateTime<Utc>) -> Result<(), crate::error::HlaviError> {
        if let Some(end) = self.end_date {
            if date > end {
                return Err(crate::error::HlaviError::InvalidDateRange {
                    start: date.to_rfc3339(),
                    end: end.to_rfc3339(),
                });
            }
        }
        self.start_date = Some(date);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Sets the end date with validation against start_date
    pub fn set_end_date(&mut self, date: DateTime<Utc>) -> Result<(), crate::error::HlaviError> {
        if let Some(start) = self.start_date {
            if date < start {
                return Err(crate::error::HlaviError::InvalidDateRange {
                    start: start.to_rfc3339(),
                    end: date.to_rfc3339(),
                });
            }
        }
        self.end_date = Some(date);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Clears the start date
    pub fn clear_start_date(&mut self) {
        self.start_date = None;
        self.updated_at = Utc::now();
    }

    /// Clears the end date
    pub fn clear_end_date(&mut self) {
        self.end_date = None;
        self.updated_at = Utc::now();
    }

    /// Sets both dates atomically with validation
    pub fn set_date_range(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<(), crate::error::HlaviError> {
        if start > end {
            return Err(crate::error::HlaviError::InvalidDateRange {
                start: start.to_rfc3339(),
                end: end.to_rfc3339(),
            });
        }
        self.start_date = Some(start);
        self.end_date = Some(end);
        self.updated_at = Utc::now();
        Ok(())
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
        assert_eq!(id.as_str(), "HLA1");

        let id = TicketId::new(42);
        assert_eq!(id.as_str(), "HLA42");

        let id = TicketId::new(1000);
        assert_eq!(id.as_str(), "HLA1000");
    }

    #[test]
    fn test_ticket_id_parsing() {
        let id = TicketId::from_str("HLA1").unwrap();
        assert_eq!(id.as_str(), "HLA1");

        let id = TicketId::from_str("HLA123").unwrap();
        assert_eq!(id.as_str(), "HLA123");

        assert!(TicketId::from_str("INVALID").is_err());
        assert!(TicketId::from_str("HLA").is_err());
        assert!(TicketId::from_str("HLAabc").is_err());
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

    #[test]
    fn test_acceptance_criteria_mark_completed() {
        let mut ac = AcceptanceCriteria::new(1, "Test AC".to_string());

        assert!(!ac.completed);
        assert!(ac.completed_at.is_none());

        ac.mark_completed();

        assert!(ac.completed);
        assert!(ac.completed_at.is_some());
    }

    #[test]
    fn test_acceptance_criteria_mark_incomplete() {
        let mut ac = AcceptanceCriteria::new(1, "Test AC".to_string());

        // First mark as completed
        ac.mark_completed();
        assert!(ac.completed);
        assert!(ac.completed_at.is_some());

        // Then mark as incomplete
        ac.mark_incomplete();

        assert!(!ac.completed);
        assert!(ac.completed_at.is_none());
    }

    #[test]
    fn test_acceptance_criteria_toggle() {
        let mut ac = AcceptanceCriteria::new(1, "Test AC".to_string());

        // Initially incomplete
        assert!(!ac.completed);
        assert!(ac.completed_at.is_none());

        // Toggle to completed
        ac.toggle();
        assert!(ac.completed);
        assert!(ac.completed_at.is_some());

        // Toggle back to incomplete
        ac.toggle();
        assert!(!ac.completed);
        assert!(ac.completed_at.is_none());
    }

    #[test]
    fn test_ticket_all_acceptance_criteria_completed() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());

        // No criteria - should return false
        assert!(!ticket.all_acceptance_criteria_completed());

        // Add criteria
        ticket.add_acceptance_criterion("AC 1".to_string());
        ticket.add_acceptance_criterion("AC 2".to_string());

        // None completed
        assert!(!ticket.all_acceptance_criteria_completed());

        // Complete one
        ticket.acceptance_criteria[0].mark_completed();
        assert!(!ticket.all_acceptance_criteria_completed());

        // Complete all
        ticket.acceptance_criteria[1].mark_completed();
        assert!(ticket.all_acceptance_criteria_completed());
    }

    #[test]
    fn test_set_start_date() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let start = Utc::now();
        assert!(ticket.set_start_date(start).is_ok());
        assert_eq!(ticket.start_date, Some(start));
    }

    #[test]
    fn test_set_end_date() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let end = Utc::now();
        assert!(ticket.set_end_date(end).is_ok());
        assert_eq!(ticket.end_date, Some(end));
    }

    #[test]
    fn test_set_date_range_valid() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);
        assert!(ticket.set_date_range(start, end).is_ok());
        assert_eq!(ticket.start_date, Some(start));
        assert_eq!(ticket.end_date, Some(end));
    }

    #[test]
    fn test_set_date_range_invalid() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start - chrono::Duration::days(1);
        assert!(ticket.set_date_range(start, end).is_err());
    }

    #[test]
    fn test_set_start_date_validates_against_existing_end() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let end = Utc::now();
        let invalid_start = end + chrono::Duration::days(1);
        ticket.set_end_date(end).unwrap();
        assert!(ticket.set_start_date(invalid_start).is_err());
    }

    #[test]
    fn test_set_end_date_validates_against_existing_start() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let start = Utc::now();
        let invalid_end = start - chrono::Duration::days(1);
        ticket.set_start_date(start).unwrap();
        assert!(ticket.set_end_date(invalid_end).is_err());
    }

    #[test]
    fn test_set_start_date_same_as_end_date() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let date = Utc::now();
        ticket.set_end_date(date).unwrap();
        assert!(ticket.set_start_date(date).is_ok());
    }

    #[test]
    fn test_clear_dates() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);

        ticket.set_date_range(start, end).unwrap();
        assert!(ticket.start_date.is_some());
        assert!(ticket.end_date.is_some());

        ticket.clear_start_date();
        assert!(ticket.start_date.is_none());

        ticket.clear_end_date();
        assert!(ticket.end_date.is_none());
    }

    #[test]
    fn test_date_setters_update_updated_at() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let initial_updated_at = ticket.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        ticket.set_start_date(Utc::now()).unwrap();
        assert!(ticket.updated_at > initial_updated_at);
    }

    #[test]
    fn test_ticket_serialization_with_dates() {
        let mut ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);
        ticket.set_date_range(start, end).unwrap();

        let json = serde_json::to_string(&ticket).unwrap();
        let deserialized: Ticket = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.start_date, Some(start));
        assert_eq!(deserialized.end_date, Some(end));
    }

    #[test]
    fn test_ticket_serialization_without_dates() {
        let ticket = Ticket::new(TicketId::new(1), "Test".to_string());
        let json = serde_json::to_string(&ticket).unwrap();

        // Fields should be omitted due to skip_serializing_if
        assert!(!json.contains("start_date"));
        assert!(!json.contains("end_date"));
    }

    #[test]
    fn test_backwards_compatibility_deserialization() {
        let old_json = r#"{
        "id": "HLA1",
        "title": "Old Ticket",
        "description": null,
        "status": "new",
        "acceptance_criteria": [],
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "agent_assigned": false,
        "rejection_reason": null
    }"#;

        let ticket: Ticket = serde_json::from_str(old_json).unwrap();
        assert_eq!(ticket.id.as_str(), "HLA1");
        assert!(ticket.start_date.is_none());
        assert!(ticket.end_date.is_none());
    }
}
