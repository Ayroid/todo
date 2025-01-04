// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "A simple CLI todo app")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new task
    Add {
        /// The title of the task
        #[arg(short, long)]
        title: String,

        /// Optional description of the task
        #[arg(short, long)]
        description: Option<String>,

        /// Due date in YYYY-MM-DD format
        #[arg(short, long)]
        due: Option<String>,
    },

    /// List all tasks
    List {
        /// Filter by completion status
        #[arg(short, long)]
        completed: Option<bool>,

        /// Sort by due date
        #[arg(short, long)]
        sort_by_due: bool,
    },

    /// View details of a specific task
    View {
        /// The ID of the task to view
        #[arg(short, long)]
        id: u32,
    },

    /// Mark a task as complete
    Complete {
        /// The ID of the task to complete
        #[arg(short, long)]
        id: u32,
    },

    /// Edit an existing task
    Edit {
        /// The ID of the task to edit
        #[arg(short, long)]
        id: u32,

        /// New title for the task
        #[arg(short, long)]
        title: Option<String>,

        /// New description for the task
        #[arg(short, long)]
        description: Option<String>,

        /// New due date in YYYY-MM-DD format
        #[arg(short, long)]
        due: Option<String>,
    },

    /// Delete a task
    Delete {
        /// The ID of the task to delete
        #[arg(short, long)]
        id: u32,
    },
}
