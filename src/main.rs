mod storage;
mod task;

use std::env;
use storage::TaskStorage;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut storage = TaskStorage::new();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            if let Some(title) = args.get(2) {
                let task = storage.add_task(title.to_string());
                println!("Added task #{}: {}", task.id, task.title);
            } else {
                println!("Please provide a task title!");
            }
        }
        Some("list") => {
            let tasks = storage.list_tasks();
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for task in tasks {
                    println!("#{}: {} {}", task.id, task.title, if task.completed { '🟢' } else { '🔴' });
                }
            }
        }
        Some("complete") => {
            if let Some(id_str) = args.get(2) {
                if let Ok(id) = id_str.parse::<u32>() {
                    if storage.complete_task(id) {
                        println!("Task #{} marked as complete!", id);
                    } else {
                        println!("Task not found!");
                    }
                } else {
                    println!("Please provide a valid task ID");
                }
            } else {
                println!("Please provide a task ID");
            }
        }
        _ => {
            println!("Usage:");
            println!(" todo add <title> - Add new task");
            println!(" todo list - List all tasks");
        }
    }
}
