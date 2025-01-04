use crate::task::Task;
use std::fs;

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
        storage.load_tasks();
        storage
    }

    fn load_tasks(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.file_path) {
            if let Ok(loaded_tasks) = serde_json::from_str(&content) {
                self.tasks = loaded_tasks;
            }
        }
    }

    fn save_tasks(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.tasks) {
            let _ = fs::write(&self.file_path, json);
        }
    }

    pub fn add_task(&mut self, title: String) -> Task {
        let id = self.next_id();
        let task = Task::new(id, title);
        self.tasks.push(task.clone());
        self.save_tasks();
        task
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn next_id(&self) -> u32 {
        self.tasks.len() as u32 + 1
    }
}
