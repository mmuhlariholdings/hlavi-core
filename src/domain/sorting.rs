use crate::domain::ticket::{Ticket, TicketStatus};
use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::str::FromStr;

/// Fields available for sorting tickets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Id,
    Title,
    Status,
    Created,
    Updated,
    Start,
    End,
    AcProgress,
    AcCount,
}

/// Sort order direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl FromStr for SortField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "id" => Ok(SortField::Id),
            "title" => Ok(SortField::Title),
            "status" => Ok(SortField::Status),
            "created" => Ok(SortField::Created),
            "updated" => Ok(SortField::Updated),
            "start" => Ok(SortField::Start),
            "end" => Ok(SortField::End),
            "ac-progress" => Ok(SortField::AcProgress),
            "ac-count" => Ok(SortField::AcCount),
            _ => Err(format!(
                "Invalid sort field '{}'. Valid fields: id, title, status, created, updated, start, end, ac-progress, ac-count",
                s
            )),
        }
    }
}

impl FromStr for SortOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(SortOrder::Ascending),
            "desc" => Ok(SortOrder::Descending),
            _ => Err(format!(
                "Invalid sort order '{}'. Valid orders: asc, desc",
                s
            )),
        }
    }
}

/// Main sorting function for tickets
///
/// Sorts a vector of tickets in-place based on the specified field and order.
///
/// # Arguments
/// * `tickets` - Mutable reference to the vector of tickets to sort
/// * `field` - The field to sort by
/// * `order` - The sort direction (ascending or descending)
///
/// # Examples
/// ```
/// use hlavi_core::domain::sorting::{sort_tickets, SortField, SortOrder};
/// use hlavi_core::domain::ticket::{Ticket, TicketId};
///
/// let mut tickets = vec![
///     Ticket::new(TicketId::new(3), "C".to_string()),
///     Ticket::new(TicketId::new(1), "A".to_string()),
///     Ticket::new(TicketId::new(2), "B".to_string()),
/// ];
///
/// sort_tickets(&mut tickets, SortField::Id, SortOrder::Ascending);
/// assert_eq!(tickets[0].id.as_str(), "HLA1");
/// ```
pub fn sort_tickets(tickets: &mut [Ticket], field: SortField, order: SortOrder) {
    tickets.sort_by(|a, b| {
        let cmp = match field {
            SortField::Id => a.id.as_str().cmp(b.id.as_str()),
            SortField::Title => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
            SortField::Status => compare_status(&a.status, &b.status),
            SortField::Created => a.created_at.cmp(&b.created_at),
            SortField::Updated => a.updated_at.cmp(&b.updated_at),
            SortField::Start => compare_option_dates(a.start_date, b.start_date),
            SortField::End => compare_option_dates(a.end_date, b.end_date),
            SortField::AcProgress => compare_ac_progress(a, b),
            SortField::AcCount => a
                .acceptance_criteria
                .len()
                .cmp(&b.acceptance_criteria.len()),
        };

        match order {
            SortOrder::Ascending => cmp,
            SortOrder::Descending => cmp.reverse(),
        }
    });
}

/// Compare ticket status by logical workflow progression
///
/// Status order: New → Open → InProgress → Pending → Review → Done → Closed
fn compare_status(a: &TicketStatus, b: &TicketStatus) -> Ordering {
    fn status_order(s: &TicketStatus) -> u8 {
        match s {
            TicketStatus::New => 0,
            TicketStatus::Open => 1,
            TicketStatus::InProgress => 2,
            TicketStatus::Pending => 3,
            TicketStatus::Review => 4,
            TicketStatus::Done => 5,
            TicketStatus::Closed => 6,
        }
    }
    status_order(a).cmp(&status_order(b))
}

/// Compare Option<DateTime> with None always sorting to end
///
/// When sorting dates, tickets with dates (Some) always come before
/// tickets without dates (None), regardless of sort order.
fn compare_option_dates(a: Option<DateTime<Utc>>, b: Option<DateTime<Utc>>) -> Ordering {
    match (a, b) {
        (Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
        (Some(_), None) => Ordering::Less, // Some comes before None
        (None, Some(_)) => Ordering::Greater, // None comes after Some
        (None, None) => Ordering::Equal,
    }
}

/// Compare by acceptance criteria completion percentage
///
/// Calculates completion percentage as (completed / total) for each ticket.
/// Tickets with no acceptance criteria are treated as 0% complete.
fn compare_ac_progress(a: &Ticket, b: &Ticket) -> Ordering {
    fn progress_pct(t: &Ticket) -> f64 {
        if t.acceptance_criteria.is_empty() {
            0.0
        } else {
            let completed = t
                .acceptance_criteria
                .iter()
                .filter(|ac| ac.completed)
                .count();
            (completed as f64) / (t.acceptance_criteria.len() as f64)
        }
    }

    progress_pct(a)
        .partial_cmp(&progress_pct(b))
        .unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ticket::TicketId;

    #[test]
    fn test_sort_tickets_by_id_ascending() {
        let mut tickets = vec![
            Ticket::new(TicketId::new(3), "C".to_string()),
            Ticket::new(TicketId::new(1), "A".to_string()),
            Ticket::new(TicketId::new(2), "B".to_string()),
        ];

        sort_tickets(&mut tickets, SortField::Id, SortOrder::Ascending);

        assert_eq!(tickets[0].id.as_str(), "HLA1");
        assert_eq!(tickets[1].id.as_str(), "HLA2");
        assert_eq!(tickets[2].id.as_str(), "HLA3");
    }

    #[test]
    fn test_sort_tickets_by_id_descending() {
        let mut tickets = vec![
            Ticket::new(TicketId::new(1), "A".to_string()),
            Ticket::new(TicketId::new(2), "B".to_string()),
            Ticket::new(TicketId::new(3), "C".to_string()),
        ];

        sort_tickets(&mut tickets, SortField::Id, SortOrder::Descending);

        assert_eq!(tickets[0].id.as_str(), "HLA3");
        assert_eq!(tickets[1].id.as_str(), "HLA2");
        assert_eq!(tickets[2].id.as_str(), "HLA1");
    }

    #[test]
    fn test_sort_tickets_by_title() {
        let mut tickets = vec![
            Ticket::new(TicketId::new(1), "Charlie".to_string()),
            Ticket::new(TicketId::new(2), "Alpha".to_string()),
            Ticket::new(TicketId::new(3), "Bravo".to_string()),
        ];

        sort_tickets(&mut tickets, SortField::Title, SortOrder::Ascending);

        assert_eq!(tickets[0].title, "Alpha");
        assert_eq!(tickets[1].title, "Bravo");
        assert_eq!(tickets[2].title, "Charlie");
    }

    #[test]
    fn test_sort_tickets_by_title_descending() {
        let mut tickets = vec![
            Ticket::new(TicketId::new(1), "Alpha".to_string()),
            Ticket::new(TicketId::new(2), "Charlie".to_string()),
            Ticket::new(TicketId::new(3), "Bravo".to_string()),
        ];

        sort_tickets(&mut tickets, SortField::Title, SortOrder::Descending);

        assert_eq!(tickets[0].title, "Charlie");
        assert_eq!(tickets[1].title, "Bravo");
        assert_eq!(tickets[2].title, "Alpha");
    }

    #[test]
    fn test_sort_tickets_by_title_case_insensitive() {
        let mut tickets = vec![
            Ticket::new(TicketId::new(1), "zebra".to_string()),
            Ticket::new(TicketId::new(2), "Apple".to_string()),
            Ticket::new(TicketId::new(3), "BANANA".to_string()),
        ];

        sort_tickets(&mut tickets, SortField::Title, SortOrder::Ascending);

        assert_eq!(tickets[0].title, "Apple");
        assert_eq!(tickets[1].title, "BANANA");
        assert_eq!(tickets[2].title, "zebra");
    }

    #[test]
    fn test_compare_status_ordering() {
        let new = TicketStatus::New;
        let open = TicketStatus::Open;
        let in_progress = TicketStatus::InProgress;
        let done = TicketStatus::Done;
        let closed = TicketStatus::Closed;

        assert_eq!(compare_status(&new, &open), Ordering::Less);
        assert_eq!(compare_status(&open, &in_progress), Ordering::Less);
        assert_eq!(compare_status(&done, &new), Ordering::Greater);
        assert_eq!(compare_status(&closed, &done), Ordering::Greater);
        assert_eq!(compare_status(&new, &new), Ordering::Equal);
    }

    #[test]
    fn test_compare_option_dates() {
        let now = Utc::now();
        let later = now + chrono::Duration::days(1);

        // Both Some: compare normally
        assert_eq!(compare_option_dates(Some(now), Some(later)), Ordering::Less);
        assert_eq!(
            compare_option_dates(Some(later), Some(now)),
            Ordering::Greater
        );

        // Some vs None: Some always comes first
        assert_eq!(compare_option_dates(Some(now), None), Ordering::Less);
        assert_eq!(compare_option_dates(None, Some(now)), Ordering::Greater);

        // Both None: equal
        assert_eq!(compare_option_dates(None, None), Ordering::Equal);
    }

    #[test]
    fn test_compare_ac_progress() {
        let mut ticket1 = Ticket::new(TicketId::new(1), "Ticket 1".to_string());
        let mut ticket2 = Ticket::new(TicketId::new(2), "Ticket 2".to_string());
        let ticket3 = Ticket::new(TicketId::new(3), "Ticket 3".to_string());

        // ticket1: 50% (1/2)
        ticket1.add_acceptance_criterion("AC1".to_string());
        ticket1.add_acceptance_criterion("AC2".to_string());
        ticket1.acceptance_criteria[0].mark_completed();

        // ticket2: 100% (1/1)
        ticket2.add_acceptance_criterion("AC1".to_string());
        ticket2.acceptance_criteria[0].mark_completed();

        // ticket3: 0% (no ACs)

        assert_eq!(compare_ac_progress(&ticket3, &ticket1), Ordering::Less);
        assert_eq!(compare_ac_progress(&ticket1, &ticket2), Ordering::Less);
        assert_eq!(compare_ac_progress(&ticket2, &ticket1), Ordering::Greater);
    }

    #[test]
    fn test_sort_by_ac_count() {
        let mut ticket1 = Ticket::new(TicketId::new(1), "Ticket 1".to_string());
        let mut ticket2 = Ticket::new(TicketId::new(2), "Ticket 2".to_string());
        let ticket3 = Ticket::new(TicketId::new(3), "Ticket 3".to_string());

        ticket1.add_acceptance_criterion("AC1".to_string());
        ticket1.add_acceptance_criterion("AC2".to_string());
        ticket1.add_acceptance_criterion("AC3".to_string());

        ticket2.add_acceptance_criterion("AC1".to_string());

        let mut tickets = vec![ticket1, ticket2, ticket3];

        sort_tickets(&mut tickets, SortField::AcCount, SortOrder::Ascending);

        assert_eq!(tickets[0].acceptance_criteria.len(), 0);
        assert_eq!(tickets[1].acceptance_criteria.len(), 1);
        assert_eq!(tickets[2].acceptance_criteria.len(), 3);
    }

    #[test]
    fn test_sort_by_dates_with_none_values() {
        let mut ticket1 = Ticket::new(TicketId::new(1), "Has both dates".to_string());
        let mut ticket2 = Ticket::new(TicketId::new(2), "Has start only".to_string());
        let ticket3 = Ticket::new(TicketId::new(3), "Has no dates".to_string());

        let early_date = Utc::now();
        let later_date = early_date + chrono::Duration::days(5);

        ticket1
            .set_date_range(early_date, later_date)
            .expect("Failed to set dates");
        ticket2
            .set_start_date(later_date)
            .expect("Failed to set start date");

        let mut tickets = vec![ticket3.clone(), ticket2.clone(), ticket1.clone()];

        sort_tickets(&mut tickets, SortField::Start, SortOrder::Ascending);

        // Tickets with dates should come before tickets without dates
        assert!(tickets[0].start_date.is_some());
        assert!(tickets[1].start_date.is_some());
        assert!(tickets[2].start_date.is_none());
    }
}
