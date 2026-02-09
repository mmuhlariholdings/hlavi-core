use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Unique identifier for a task (e.g., HLA1, HLA2, HLA100)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(String);

impl TaskId {
    // Default prefix for task IDs (could be made configurable in the future)
    const DEFAULT_PREFIX: &'static str = "HLA";

    /// Creates a new TaskId from a counter
    pub fn new(counter: u32) -> Self {
        Self(format!("{}{}", Self::DEFAULT_PREFIX, counter))
    }

    /// Returns the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for TaskId {
    type Err = crate::error::HlaviError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert to uppercase for case-insensitive comparison
        let normalized = s.to_uppercase();
        let prefix = TaskId::DEFAULT_PREFIX;

        if normalized.starts_with(prefix) && normalized.len() > prefix.len() {
            // Verify the rest is a valid number
            if normalized[prefix.len()..].parse::<u32>().is_ok() {
                // Store the normalized (uppercase) form
                Ok(Self(normalized))
            } else {
                Err(crate::error::HlaviError::InvalidTaskId(s.to_string()))
            }
        } else {
            Err(crate::error::HlaviError::InvalidTaskId(s.to_string()))
        }
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Status of a task on the kanban board
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    New,
    Open,
    InProgress,
    Pending,
    Review,
    Done,
    Closed,
}

impl fmt::Display for TaskStatus {
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

impl TaskStatus {
    /// Checks if a status transition is valid
    pub fn can_transition_to(&self, target: &TaskStatus) -> bool {
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

/// Acceptance criteria for a task
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

/// A kanban task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
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

impl Task {
    /// Creates a new task with the given ID and title
    pub fn new(id: TaskId, title: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description: None,
            status: TaskStatus::New,
            acceptance_criteria: Vec::new(),
            created_at: now,
            updated_at: now,
            agent_assigned: false,
            rejection_reason: None,
            start_date: None,
            end_date: None,
        }
    }

    /// Sets the title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
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
    pub fn set_date_range(
        &mut self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<(), crate::error::HlaviError> {
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

    /// Changes the task status
    pub fn transition_to(
        &mut self,
        new_status: TaskStatus,
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

    /// Checks if the task can be marked as done
    pub fn can_mark_done(&self) -> bool {
        self.status == TaskStatus::Review && self.all_acceptance_criteria_completed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_id_creation() {
        let id = TaskId::new(1);
        assert_eq!(id.as_str(), "HLA1");

        let id = TaskId::new(42);
        assert_eq!(id.as_str(), "HLA42");

        let id = TaskId::new(1000);
        assert_eq!(id.as_str(), "HLA1000");
    }

    #[test]
    fn test_task_id_parsing() {
        let id = TaskId::from_str("HLA1").unwrap();
        assert_eq!(id.as_str(), "HLA1");

        let id = TaskId::from_str("HLA123").unwrap();
        assert_eq!(id.as_str(), "HLA123");

        assert!(TaskId::from_str("INVALID").is_err());
        assert!(TaskId::from_str("HLA").is_err());
        assert!(TaskId::from_str("HLAabc").is_err());
    }

    #[test]
    fn test_task_id_parsing_case_insensitive() {
        // Lowercase
        let id = TaskId::from_str("hla1").unwrap();
        assert_eq!(id.as_str(), "HLA1");

        let id = TaskId::from_str("hla42").unwrap();
        assert_eq!(id.as_str(), "HLA42");

        // Mixed case
        let id = TaskId::from_str("Hla123").unwrap();
        assert_eq!(id.as_str(), "HLA123");

        let id = TaskId::from_str("HlA99").unwrap();
        assert_eq!(id.as_str(), "HLA99");

        let id = TaskId::from_str("hLa5").unwrap();
        assert_eq!(id.as_str(), "HLA5");

        // All variations should normalize to uppercase
        assert_eq!(
            TaskId::from_str("hla1").unwrap(),
            TaskId::from_str("HLA1").unwrap()
        );
        assert_eq!(
            TaskId::from_str("Hla1").unwrap(),
            TaskId::from_str("HLA1").unwrap()
        );
    }

    #[test]
    fn test_status_transitions() {
        assert!(TaskStatus::New.can_transition_to(&TaskStatus::Open));
        assert!(TaskStatus::Open.can_transition_to(&TaskStatus::InProgress));
        assert!(!TaskStatus::New.can_transition_to(&TaskStatus::Done));
    }

    #[test]
    fn test_task_acceptance_criteria() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());

        task.add_acceptance_criterion("AC 1".to_string());
        task.add_acceptance_criterion("AC 2".to_string());

        assert_eq!(task.acceptance_criteria.len(), 2);
        assert!(!task.all_acceptance_criteria_completed());
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
    fn test_task_all_acceptance_criteria_completed() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());

        // No criteria - should return false
        assert!(!task.all_acceptance_criteria_completed());

        // Add criteria
        task.add_acceptance_criterion("AC 1".to_string());
        task.add_acceptance_criterion("AC 2".to_string());

        // None completed
        assert!(!task.all_acceptance_criteria_completed());

        // Complete one
        task.acceptance_criteria[0].mark_completed();
        assert!(!task.all_acceptance_criteria_completed());

        // Complete all
        task.acceptance_criteria[1].mark_completed();
        assert!(task.all_acceptance_criteria_completed());
    }

    #[test]
    fn test_set_title() {
        let mut task = Task::new(TaskId::new(1), "Original Title".to_string());
        assert_eq!(task.title, "Original Title");

        task.set_title("Updated Title".to_string());
        assert_eq!(task.title, "Updated Title");
    }

    #[test]
    fn test_set_title_updates_updated_at() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let initial_updated_at = task.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        task.set_title("New Title".to_string());

        assert!(task.updated_at > initial_updated_at);
    }

    #[test]
    fn test_set_start_date() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let start = Utc::now();
        assert!(task.set_start_date(start).is_ok());
        assert_eq!(task.start_date, Some(start));
    }

    #[test]
    fn test_set_end_date() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let end = Utc::now();
        assert!(task.set_end_date(end).is_ok());
        assert_eq!(task.end_date, Some(end));
    }

    #[test]
    fn test_set_date_range_valid() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);
        assert!(task.set_date_range(start, end).is_ok());
        assert_eq!(task.start_date, Some(start));
        assert_eq!(task.end_date, Some(end));
    }

    #[test]
    fn test_set_date_range_invalid() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start - chrono::Duration::days(1);
        assert!(task.set_date_range(start, end).is_err());
    }

    #[test]
    fn test_set_start_date_validates_against_existing_end() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let end = Utc::now();
        let invalid_start = end + chrono::Duration::days(1);
        task.set_end_date(end).unwrap();
        assert!(task.set_start_date(invalid_start).is_err());
    }

    #[test]
    fn test_set_end_date_validates_against_existing_start() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let start = Utc::now();
        let invalid_end = start - chrono::Duration::days(1);
        task.set_start_date(start).unwrap();
        assert!(task.set_end_date(invalid_end).is_err());
    }

    #[test]
    fn test_set_start_date_same_as_end_date() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let date = Utc::now();
        task.set_end_date(date).unwrap();
        assert!(task.set_start_date(date).is_ok());
    }

    #[test]
    fn test_clear_dates() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);

        task.set_date_range(start, end).unwrap();
        assert!(task.start_date.is_some());
        assert!(task.end_date.is_some());

        task.clear_start_date();
        assert!(task.start_date.is_none());

        task.clear_end_date();
        assert!(task.end_date.is_none());
    }

    #[test]
    fn test_date_setters_update_updated_at() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let initial_updated_at = task.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        task.set_start_date(Utc::now()).unwrap();
        assert!(task.updated_at > initial_updated_at);
    }

    #[test]
    fn test_task_serialization_with_dates() {
        let mut task = Task::new(TaskId::new(1), "Test".to_string());
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);
        task.set_date_range(start, end).unwrap();

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.start_date, Some(start));
        assert_eq!(deserialized.end_date, Some(end));
    }

    #[test]
    fn test_task_serialization_without_dates() {
        let task = Task::new(TaskId::new(1), "Test".to_string());
        let json = serde_json::to_string(&task).unwrap();

        // Fields should be omitted due to skip_serializing_if
        assert!(!json.contains("start_date"));
        assert!(!json.contains("end_date"));
    }

    #[test]
    fn test_backwards_compatibility_deserialization() {
        let old_json = r#"{
        "id": "HLA1",
        "title": "Old Task",
        "description": null,
        "status": "new",
        "acceptance_criteria": [],
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "agent_assigned": false,
        "rejection_reason": null
    }"#;

        let task: Task = serde_json::from_str(old_json).unwrap();
        assert_eq!(task.id.as_str(), "HLA1");
        assert!(task.start_date.is_none());
        assert!(task.end_date.is_none());
    }
}
