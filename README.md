# Rust Todo CLI

A simple, colorful command-line todo list manager written in Rust.

## Features

- **Add tasks**: Quickly add new tasks to your list.
- **List tasks**: View all tasks, or filter by pending/done status.
- **Mark as done**: Mark tasks as completed.
- **Remove tasks**: Delete specific tasks.
- **Clear all**: Remove all tasks from the list.
- **Persistent storage**: Tasks are saved to a JSON file in your home directory (`~/.todo-cli/todo.json`).
- **Colored output**: Visual feedback for actions and status.

## Installation

Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust-todo-cli
   ```

2. Build and install:
   ```bash
   cargo install --path .
   ```

   Or run directly with `cargo run`:
   ```bash
   cargo run -- --help
   ```

## Usage

### Add a Task
```bash
todo add "Buy groceries"
```

### List Tasks
Show all tasks:
```bash
todo list
```

Show only pending tasks:
```bash
todo list --pending
```

Show only completed tasks:
```bash
todo list --done
```

### Mark a Task as Done
Provide the ID of the task (seen in `list` output):
```bash
todo done 1
```

### Remove a Task
```bash
todo remove 1
```

### Clear All Tasks
```bash
todo clear
```

## Data Storage

Your tasks are stored in `~/.todo-cli/todo.json`.

## Development

### Running Tests
Run the unit tests with:
```bash
cargo test
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [serde](https://crates.io/crates/serde) & [serde_json](https://crates.io/crates/serde_json) - Serialization
- [colored](https://crates.io/crates/colored) - Terminal colors
- [dirs](https://crates.io/crates/dirs) - Platform-agnostic home directory discovery
- [anyhow](https://crates.io/crates/anyhow) - Error handling
- [tempfile](https://crates.io/crates/tempfile) - Temporary file creation (Dev dependency)
