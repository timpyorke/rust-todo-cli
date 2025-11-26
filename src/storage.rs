use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{Context, Result};

use crate::constants::*;
use crate::models::task::Priority;
use crate::models::task::Task;

pub fn load_tasks(path: &str) -> Result<Vec<Task>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path).context(ERR_OPEN_FILE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).context(ERR_PARSE_FILE)?;
    Ok(tasks)
}

pub fn matches_search(task: &Task, keyword: &str) -> bool {
    let keyword = keyword.to_lowercase();
    task.text.to_lowercase().contains(&keyword)
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .context(ERR_OPEN_FILE_WRITE)?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).context(ERR_WRITE_FILE)?;
    Ok(())
}

pub fn next_id(tasks: &[Task]) -> usize {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", "No tasks to show. 🎉".green());
        return;
    }

    println!("{}", "Your tasks:".bold());
    for t in tasks {
        let status = if t.done {
            "[x]".green()
        } else {
            "[ ]".yellow()
        };

        let prio_label = match t.priority {
            Priority::High => "HIGH".red().bold(),
            Priority::Normal => "NORM".white(),
            Priority::Low => "LOW".cyan(),
        };

        let due_label = t
            .due
            .map(|d| format!(" due {}", d.format("%Y-%m-%d")))
            .unwrap_or_default();
        let tags_label = if t.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", t.tags.join(","))
        };

        println!(
            "{} {} [{}] {}",
            format!("{:>3}.", t.id).cyan(),
            status,
            prio_label,
            format!("{}{}{}", t.text, due_label, tags_label)
        );
    }

    let done_count = tasks.iter().filter(|t| t.done).count();
    let pending_count = tasks.len() - done_count;
    println!(
        "\n{}  {}",
        format!("Done: {done_count}").green(),
        format!("Pending: {pending_count}").yellow()
    );
}

#[cfg(test)]
mod tests;
