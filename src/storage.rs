use chrono::{DateTime, Utc};
use crate::error::{Result, TodoError};
use crate::task::Task;
use std::fs;
use std::path::Path;

pub struct TaskStorage {
    tasks: Vec<Task>,
    file_path: String,
}

impl TaskStorage {
    pub fn new() -> TaskStorage {
        let mut storage = TaskStorage {
            tasks: Vec::new(),
            file_path: String::from("tasks.json"),
        };
        if let Ok(loaded_tasks) = storage.load_tasks() {
            storage.tasks = loaded_tasks;
        }
        storage
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        if !Path::new(&self.file_path).exists() {
            fs::write(&self.file_path, "[]")?;
        }
        let content = fs::read_to_string(&self.file_path)?;
        let tasks = serde_json::from_str(&content)?;
        Ok(tasks)
    }

    pub fn get_task(&self, id: u32) -> Result<&Task> {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))
    }

    fn save_tasks(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<Task> {
        let id = self.next_id();
        let task = Task::new(id, title, description, due_date);
        self.tasks.push(task.clone());
        self.save_tasks()?;
        Ok(task)
    }

    pub fn complete_task(&mut self, id: u32) -> Result<()> {
        let task = self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        task.completed = true;
        self.save_tasks()?;
        Ok(())
    }

    pub fn edit_task(
        &mut self,
        id: u32,
        title: Option<String>,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<()> {
        let task = self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        if let Some(title) = title {
            task.title = title;
        }
        if let Some(desc) = description {
            task.description = Some(desc);
        }
        task.due_date = due_date;

        self.save_tasks()?;
        Ok(())
    }

    pub fn delete_task(&mut self, id: u32) -> Result<()> {
        let pos = self.tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        self.tasks.remove(pos);
        self.save_tasks()?;
        Ok(())
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn next_id(&self) -> u32 {
        self.tasks.len() as u32 + 1
    }
}