# Rust Todo CLI

A powerful command-line todo application built with Rust, featuring persistent storage and intuitive commands.

## Features

- ✨ Create tasks with titles and optional descriptions
- 📋 List all tasks with status indicators
- 🔍 View detailed task information
- ✅ Mark tasks as complete
- 🗑️ Delete tasks
- 💾 Persistent JSON storage
- ⏰ Automatic timestamp tracking

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/todo
cd todo

# Install using Cargo
cargo install --path .
```

### From crates.io
```bash
cargo install todo
```

## Usage

### Basic Commands

```bash
# Add a new task
todo add -t "Learn Rust" -d "Complete the todo app project"
todo add --title "Buy groceries" --description "Milk, eggs, bread"

# List all tasks
todo list

# View a specific task
todo view -i 1
todo view --id 1

# Complete a task
todo complete -i 1
todo complete --id 1

# Delete a task
todo delete -i 1
todo delete --id 1
```

### Command Details

#### Add a Task
```bash
todo add -t <title> [-d <description>]

Options:
  -t, --title         Task title (required)
  -d, --description   Task description (optional)
```

#### List Tasks
```bash
todo list

# Output format:
#1 [ ] Task title
#2 [✓] Completed task
```

#### View Task Details
```bash
todo view -i <id>

# Output includes:
# - Task ID
# - Title
# - Description (if any)
# - Status
# - Creation date
# - Completion date (if completed)
```

#### Complete Task
```bash
todo complete -i <id>
```

#### Delete Task
```bash
todo delete -i <id>
```

## Development

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/todo
cd todo

# Build
cargo build
```

### Project Structure
```
todo/
├── src/
│   ├── main.rs
│   ├── cli.rs      # Command-line interface definitions
│   ├── task.rs     # Task struct and implementations
│   ├── storage.rs  # Storage handling
│   └── error.rs    # Error types
├── tests/          # Test modules
├── Cargo.toml
└── README.md
```

## Storage

Tasks are stored in a JSON file (`tasks.json`) in your current directory. Each task includes:
- Unique ID
- Title
- Optional description
- Completion status
- Creation timestamp
- Completion timestamp (if completed)

Example storage format:
```json
[
  {
    "id": 1,
    "title": "Learn Rust",
    "description": "Complete the todo app project",
    "completed": false,
    "created_at": "2024-01-05T10:00:00Z",
    "completed_at": null
  }
]
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.