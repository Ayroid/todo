// src/main.rs
mod cli;
mod error;
mod storage;
mod task;

use chrono::{DateTime, Utc};
use clap::Parser;
use cli::{Cli, Command};
use error::Result;
use storage::TaskStorage;

fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    let naive_date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| error::TodoError::ValidationError("Invalid date format".to_string()))?;

    let naive_datetime = naive_date
        .and_hms_opt(23, 59, 59)
        .ok_or_else(|| error::TodoError::ValidationError("Invalid time".to_string()))?;

    Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Utc))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut storage = TaskStorage::new();

    match cli.command {
        Command::Add {
            title,
            description,
            due,
        } => {
            let due_date = if let Some(date_str) = due {
                Some(parse_date(&date_str)?)
            } else {
                None
            };

            let task = storage.add_task(title.clone(), description, due_date)?;
            println!("Added task #{}: {}", task.id, task.title);
        }

        Command::List {
            completed,
            sort_by_due,
        } => {
            let mut tasks = storage.list_tasks().to_vec();

            if let Some(is_completed) = completed {
                tasks.retain(|task| task.completed == is_completed);
            }

            if sort_by_due {
                tasks.sort_by_key(|task| task.due_date);
            }

            if tasks.is_empty() {
                println!("No tasks found!");
            } else {
                for task in tasks {
                    let status = if task.completed { "✅" } else { "⭕" };
                    let due_str = task.due_date.map_or("No due date".to_string(), |d| {
                        d.format("%Y-%m-%d").to_string()
                    });
                    println!("{} #{}: {} (Due: {})", status, task.id, task.title, due_str);
                }
            }
        }

        Command::View { id } => {
            let task = storage.get_task(id)?;
            println!("Task #{}", task.id);
            println!("Title: {}", task.title);
            if let Some(desc) = &task.description {
                println!("Description: {}", desc);
            }
            println!(
                "Status: {}",
                if task.completed {
                    "Completed"
                } else {
                    "Pending"
                }
            );
            if let Some(due) = task.due_date {
                println!("Due: {}", due.format("%Y-%m-%d"));
            }
        }

        Command::Complete { id } => {
            storage.complete_task(id)?;
            println!("Task #{} marked as complete!", id);
        }

        Command::Edit {
            id,
            title,
            description,
            due,
        } => {
            let due_date = if let Some(date_str) = due {
                Some(parse_date(&date_str)?)
            } else {
                None
            };

            storage.edit_task(id, title, description, due_date)?;
            println!("Task #{} updated successfully!", id);
        }

        Command::Delete { id } => {
            storage.delete_task(id)?;
            println!("Task #{} deleted successfully!", id);
        }
    }

    Ok(())
}
