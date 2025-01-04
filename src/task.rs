use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            completed: false,
            created_at: Utc::now(),
            due_date,
            completed_at: None,
        }
    }
}
