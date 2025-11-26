use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task description
        text: String,
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
