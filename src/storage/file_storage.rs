use crate::{
    domain::{Board, Ticket, TicketId},
    error::{HlaviError, Result},
    storage::Storage,
};
use async_trait::async_trait;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::fs;

/// File-based storage implementation
pub struct FileStorage {
    root_path: PathBuf,
}

impl FileStorage {
    const HLAVI_DIR: &'static str = ".hlavi";
    const TICKETS_DIR: &'static str = "tickets";
    const BOARD_FILE: &'static str = "board.json";
    #[allow(dead_code)]
    const CONFIG_FILE: &'static str = "config.toml";

    /// Creates a new FileStorage instance for the given project root
    pub fn new(project_root: impl AsRef<Path>) -> Self {
        Self {
            root_path: project_root.as_ref().join(Self::HLAVI_DIR),
        }
    }

    fn tickets_dir(&self) -> PathBuf {
        self.root_path.join(Self::TICKETS_DIR)
    }

    fn board_file(&self) -> PathBuf {
        self.root_path.join(Self::BOARD_FILE)
    }

    fn ticket_file(&self, id: &TicketId) -> PathBuf {
        self.tickets_dir().join(format!("{}.json", id.as_str()))
    }

    async fn ensure_directory_exists(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn initialize(&self) -> Result<()> {
        // Create .hlavi directory structure
        self.ensure_directory_exists(&self.root_path).await?;
        self.ensure_directory_exists(&self.tickets_dir()).await?;

        // Create default board if it doesn't exist
        if !self.board_file().exists() {
            let board = Board::default();
            self.save_board(&board).await?;
        }

        // Create .gitignore
        let gitignore_path = self.root_path.join(".gitignore");
        if !gitignore_path.exists() {
            fs::write(gitignore_path, "# Local caches\n*.db\n*.db-*\n").await?;
        }

        Ok(())
    }

    async fn save_ticket(&self, ticket: &Ticket) -> Result<()> {
        self.ensure_directory_exists(&self.tickets_dir()).await?;

        let json = serde_json::to_string_pretty(ticket)?;
        let file_path = self.ticket_file(&ticket.id);

        fs::write(file_path, json).await?;
        Ok(())
    }

    async fn load_ticket(&self, id: &TicketId) -> Result<Ticket> {
        let file_path = self.ticket_file(id);

        if !file_path.exists() {
            return Err(HlaviError::TicketNotFound(id.to_string()));
        }

        let contents = fs::read_to_string(&file_path).await?;
        let ticket: Ticket = serde_json::from_str(&contents)?;

        Ok(ticket)
    }

    async fn list_ticket_ids(&self) -> Result<Vec<TicketId>> {
        let tickets_dir = self.tickets_dir();

        if !tickets_dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries = fs::read_dir(&tickets_dir).await?;
        let mut ids: Vec<TicketId> = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(id) = TicketId::from_str(stem) {
                        ids.push(id);
                    }
                }
            }
        }

        ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        Ok(ids)
    }

    async fn delete_ticket(&self, id: &TicketId) -> Result<()> {
        let file_path = self.ticket_file(id);

        if !file_path.exists() {
            return Err(HlaviError::TicketNotFound(id.to_string()));
        }

        fs::remove_file(file_path).await?;
        Ok(())
    }

    async fn save_board(&self, board: &Board) -> Result<()> {
        self.ensure_directory_exists(&self.root_path).await?;

        let json = serde_json::to_string_pretty(board)?;
        fs::write(self.board_file(), json).await?;

        Ok(())
    }

    async fn load_board(&self) -> Result<Board> {
        let board_file = self.board_file();

        if !board_file.exists() {
            return Err(HlaviError::BoardNotInitialized);
        }

        let contents = fs::read_to_string(&board_file).await?;
        let board: Board = serde_json::from_str(&contents)?;

        Ok(board)
    }

    async fn is_initialized(&self) -> bool {
        self.root_path.exists() && self.board_file().exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        assert!(!storage.is_initialized().await);

        storage.initialize().await.unwrap();

        assert!(storage.is_initialized().await);
        assert!(storage.tickets_dir().exists());
        assert!(storage.board_file().exists());
    }

    #[tokio::test]
    async fn test_ticket_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.initialize().await.unwrap();

        let ticket = Ticket::new(TicketId::new(1), "Test Ticket".to_string());
        storage.save_ticket(&ticket).await.unwrap();

        let loaded = storage.load_ticket(&ticket.id).await.unwrap();
        assert_eq!(loaded.id.as_str(), ticket.id.as_str());
        assert_eq!(loaded.title, ticket.title);
    }

    #[tokio::test]
    async fn test_ticket_with_dates_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.initialize().await.unwrap();

        let mut ticket = Ticket::new(TicketId::new(1), "Test Ticket".to_string());
        let start = chrono::Utc::now();
        let end = start + chrono::Duration::days(7);

        ticket.set_date_range(start, end).unwrap();
        storage.save_ticket(&ticket).await.unwrap();

        let loaded = storage.load_ticket(&ticket.id).await.unwrap();
        assert_eq!(loaded.start_date, Some(start));
        assert_eq!(loaded.end_date, Some(end));
    }
}
