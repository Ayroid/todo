use crate::task::Task;

pub struct TaskStorage {
    tasks: Vec<Task>,
}

impl TaskStorage {
    pub fn new() -> TaskStorage {
        TaskStorage { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, title: String) -> Task {
        let id = self.next_id();
        let task = Task::new(id, title);
        self.tasks.push(task.clone());
        task
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn next_id(&self) -> u32 {
        self.tasks.len() as u32 + 1
    }
}
