use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

mod models;
mod storage;
mod constants;

use models::{cli::Cli, commands::Commands, task::Task};
use storage::{load_tasks, next_id, print_tasks, save_tasks};
use constants::*;

fn db_file_path() -> Result<PathBuf> {
    let home = home_dir().context(ERR_HOME_DIR)?;
    let dir = home.join(DIR_NAME);

    if !dir.exists() {
        fs::create_dir_all(&dir).context(ERR_CREATE_DIR)?;
    }

    Ok(dir.join(DB_FILE_NAME))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let db_path = db_file_path()?;
    let db_path_str = db_path.to_string_lossy();

    // Load tasks from file (or empty list)
    let mut tasks = load_tasks(&db_path_str).unwrap_or_default();

    match cli.command {
        Commands::Add { text } => {
            let next_id = next_id(&tasks);
            let task = Task {
                id: next_id,
                text,
                done: false,
            };
            tasks.push(task);
            save_tasks(&db_path_str, &tasks)?;
            println!("{}", format!("Task added (id: {next_id})").green());
        }

        Commands::List { done, pending } => {
            // Decide filter based on flags
            let filtered: Vec<&Task> = match (done, pending) {
                (true, false) => tasks.iter().filter(|t| t.done).collect(),
                (false, true) => tasks.iter().filter(|t| !t.done).collect(),
                // both false OR both true -> show all
                _ => tasks.iter().collect(),
            };

            if filtered.is_empty() {
                if done {
                    println!("{}", MSG_NO_DONE_TASKS.green());
                } else if pending {
                    println!("{}", MSG_NO_PENDING_TASKS.green());
                } else {
                    println!("{}", MSG_NO_TASKS.green());
                }
            } else {
                // Map &Task → Task for print_tasks which expects &[Task]
                let owned: Vec<Task> = filtered.into_iter().cloned().collect();
                print_tasks(&owned);
            }
        }

        Commands::Done { id } => {
            let mut found = false;
            for t in &mut tasks {
                if t.id == id {
                    t.done = true;
                    found = true;
                    break;
                }
            }

            if found {
                save_tasks(&db_path_str, &tasks)?;
                println!("{}", format!("Marked task {id} as done ✅").green());
            } else {
                eprintln!("{}", format!("Task with id {id} not found.").red());
            }
        }

        Commands::Remove { id } => {
            let len_before = tasks.len();
            tasks.retain(|t| t.id != id);

            if tasks.len() < len_before {
                save_tasks(&db_path_str, &tasks)?;
                println!("{}", format!("Removed task {id} 🗑️").green());
            } else {
                eprintln!("{}", format!("Task with id {id} not found.").red());
            }
        }

        Commands::Clear => {
            tasks.clear();
            save_tasks(&db_path_str, &tasks)?;
            println!("{}", MSG_ALL_CLEARED.yellow());
        }
    }

    Ok(())
}
