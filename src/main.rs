// src/main.rs

mod cli;
mod error;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use storage::TaskStorage;
use task::Task;

use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut storage = TaskStorage::new(PathBuf::from("tasks.json"));

    match cli.command {
        Commands::Add { title, description } => {
            let tasks = storage.load_tasks()?;
            let new_id = tasks.last().map_or(1, |t| t.id + 1);
            let task = Task::new(new_id, title, description);
            storage.add_task(task)?;
            println!("Task added successfully!");
        }
        Commands::List => {
            let tasks = storage.load_tasks()?;
            if tasks.is_empty() {
                println!("No tasks found.");
                return Ok(());
            }

            for task in tasks {
                println!(
                    "#{} [{}] {} {}",
                    task.id,
                    if task.completed { "✓" } else { " " },
                    task.title,
                    task.description.unwrap_or_default()
                );
            }
        }
        Commands::View { id } => {
            let task = storage.get_task(id)?;
            println!("Task #{}", task.id);
            println!("Title: {}", task.title);
            println!("Description: {}", task.description.unwrap_or_default());
            println!("Status: {}", if task.completed { "Completed" } else { "Pending" });
            println!("Created: {}", task.created_at);
            if let Some(completed_at) = task.completed_at {
                println!("Completed: {}", completed_at);
            }
        }
        Commands::Complete { id } => {
            let mut task = storage.get_task(id)?;
            task.complete();
            storage.update_task(id, task)?;
            println!("Task #{} marked as complete!", id);
        }
        Commands::Delete { id } => {
            storage.delete_task(id)?;
            println!("Task #{} deleted successfully!", id);
        }
    }

    Ok(())
}