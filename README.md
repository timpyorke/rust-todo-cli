# Rust Todo CLI

A simple, colorful command-line todo list manager written in Rust.

## Features

- **Add tasks** with priority, due date, and tags
- **List tasks** with filters: pending/done, search, tags (must match all), and sorting by id or due date
- **Mark as done**, **edit**, **remove**, or **clear**
- **Persistent storage** in your home directory (`~/.todo-cli/todo.json`)
- **Colored output** for quick scanning

## Installation

Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

**Option A: Install from Git tag (recommended)**
```bash
cargo install --git https://github.com/timpyorke/rust-todo-cli --tag v1.0.0
```

**Option B: Download a GitHub Release binary**
- Grab the archive from the Releases page, extract, and put `todo` on your `PATH`.

**Option C: Local build (for development)**
```bash
git clone https://github.com/timpyorke/rust-todo-cli
cd rust-todo-cli
cargo install --path .
# or run without installing:
cargo run -- --help
```

## Usage

### Add a Task
```bash
# Basic
todo add "Buy groceries"

# With priority, due date (YYYY-MM-DD), and tags (comma-separated)
todo add "Pay bills" --priority high --due 2025-02-01 --tags home,finance
```

### List Tasks
- All tasks:
  ```bash
  todo list
  ```
- Only pending:
  ```bash
  todo list --pending
  ```
- Only done:
  ```bash
  todo list --done
  ```
- Search by text:
  ```bash
  todo list --search groceries
  ```
- Filter by tags (task must include all):
  ```bash
  todo list --tags home,finance
  ```
- Sort (default is id):
  ```bash
  todo list --sort date
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
- [chrono](https://crates.io/crates/chrono) - Dates for due handling
- [anyhow](https://crates.io/crates/anyhow) - Error handling
- [tempfile](https://crates.io/crates/tempfile) - Temporary file creation (Dev dependency)
