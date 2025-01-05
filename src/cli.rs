use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "A simple todo application", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    List,
    View {
        #[arg(short = 'i', long = "id")]
        id: u32,
    },
    Complete {
        #[arg(short = 'i', long = "id")]
        id: u32,
    },
    Delete {
        #[arg(short = 'i', long = "id")]
        id: u32,
    },
}
