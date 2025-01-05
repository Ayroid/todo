use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        title: String,
        #[arg[short, long]]
        description: Option<String>,
    },
    List,
    View {
        #[arg(short, long)]
        id: u32,
    },
    Complete {
        #[arg(short, long)]
        id: u32,
    },
    Delete {
        #[arg(short, long)]
        id: u32,
    },
}
