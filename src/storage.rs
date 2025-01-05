use crate::error::{Result, TodoError};
use crate::task::Task;
use std::fs;
use std::path::PathBuf;

pub struct TaskStorage {
    file_path: PathBuf,
}

impl TaskStorage {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        if !self.file_path.exists() {
            fs::write(&self.file_path, "[]")?;
        }

        let content = fs::read_to_string(&self.file_path)?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let tasks: Vec<Task> = serde_json::from_str(&content)?;
        Ok(tasks)
    }

    fn save_tasks(&self, tasks: &[Task]) -> Result<()> {
        let content = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn add_task(&mut self, task: Task) -> Result<()> {
        let mut tasks = self.load_tasks()?;
        tasks.push(task);
        self.save_tasks(&tasks)
    }

    pub fn get_task(&self, id: u32) -> Result<Task> {
        let tasks = self.load_tasks()?;
        tasks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or(TodoError::TaskNotFound(id))
    }

    pub fn update_task(&self, id: u32, updated_task: Task) -> Result<()> {
        let mut tasks = self.load_tasks()?;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            *task = updated_task;
            self.save_tasks(&tasks)?;
            Ok(())
        } else {
            Err(TodoError::TaskNotFound(id))
        }
    }

    pub fn delete_task(&self, id: u32) -> Result<()> {
        let mut tasks = self.load_tasks()?;
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(pos);
            let _ = self.save_tasks(&tasks);
            Ok(())
        } else {
            Err(TodoError::TaskNotFound(id))
        }
    }
}
