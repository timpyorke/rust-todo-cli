use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

use todo::constants::*;
use std::cmp::Ordering;

use todo::models::{cli::Cli, commands::{Commands, SortBy}, task::Task};
use todo::storage::{load_tasks, matches_search, next_id, print_tasks, save_tasks};

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
        Commands::Add {
            text,
            due,
            tags,
            priority,
        } => {
            let next_id = next_id(&tasks);
            let task = Task {
                id: next_id,
                text,
                done: false,
                due,
                tags,
                priority,
            };
            tasks.push(task);
            save_tasks(&db_path_str, &tasks)?;
            println!(
                "{}",
                format!("Task added (id: {next_id}, priority: {:?})", priority).green()
            );
        }

        Commands::List {
            done,
            pending,
            search,
            sort,
            tags,
        } => {
            // Start with full list
            let mut filtered: Vec<&Task> = tasks.iter().collect();

            // Filter: done / pending
            filtered = match (done, pending) {
                (true, false) => filtered.into_iter().filter(|t| t.done).collect(),
                (false, true) => filtered.into_iter().filter(|t| !t.done).collect(),
                _ => filtered, // both false or both true → no done/pending filter
            };

            // Filter: search keyword
            if let Some(keyword) = search {
                filtered = filtered
                    .into_iter()
                    .filter(|t| matches_search(t, &keyword))
                    .collect();
            }

            // Filter: tags (task must contain all requested tags)
            if !tags.is_empty() {
                filtered = filtered
                    .into_iter()
                    .filter(|t| {
                        tags.iter().all(|tag| {
                            t.tags
                                .iter()
                                .any(|task_tag| task_tag.eq_ignore_ascii_case(tag))
                        })
                    })
                    .collect();
            }

            // Handle empty results
            if filtered.is_empty() {
                println!("{}", "No tasks match your filters. 🔍".yellow());
                return Ok(());
            }

            // Convert &Task → Task so we can reuse print_tasks()
            let mut owned: Vec<Task> = filtered.into_iter().cloned().collect();

            // Sort
            match sort {
                SortBy::Id => owned.sort_by_key(|t| t.id),
                SortBy::Date => owned.sort_by(|a, b| match (a.due, b.due) {
                    (Some(ad), Some(bd)) => ad.cmp(&bd).then(a.id.cmp(&b.id)),
                    (Some(_), None) => Ordering::Less,   // dated tasks first
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => a.id.cmp(&b.id),
                }),
            }

            print_tasks(&owned);
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

        Commands::Edit { id, text } => {
            let mut found = false;

            for t in &mut tasks {
                if t.id == id {
                    t.text = text.clone();
                    found = true;
                    break;
                }
            }

            if found {
                save_tasks(&db_path_str, &tasks)?;
                println!("{}", format!("Updated task {id} ✏️").green());
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
