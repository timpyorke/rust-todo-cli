use chrono::NaiveDate;
use clap::{Subcommand, ValueEnum};

use crate::models::task::Priority;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task description
        text: String,

        /// Due date in YYYY-MM-DD format
        #[arg(long, value_parser = parse_due_date)]
        due: Option<NaiveDate>,

        /// Comma-separated list of tags
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Task priority (low, normal, high)
        #[arg(long, value_enum, default_value_t = Priority::Normal)]
        priority: Priority,
    },

    /// List tasks (optionally filtered)
    List {
        /// Show only done tasks
        #[arg(long)]
        done: bool,

        /// Show only pending tasks
        #[arg(long)]
        pending: bool,

        /// Search text (case-insensitive)
        #[arg(long)]
        search: Option<String>,

        /// Sort tasks by id or due date
        #[arg(long, value_enum, default_value_t = SortBy::Id)]
        sort: SortBy,

        /// Filter by tags (comma-separated). Task must include all provided tags.
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// Mark a task as done (by id)
    Done {
        /// Task id
        id: usize,
    },

    /// Edit an existing task's text
    Edit {
        /// Task id
        id: usize,

        /// New task text
        text: String,
    },

    /// Remove a task (by id)
    Remove {
        /// Task id
        id: usize,
    },

    /// Remove all tasks
    Clear,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortBy {
    Id,
    Date,
}

fn parse_due_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| "Use YYYY-MM-DD, e.g. 2025-02-01".to_string())
}
