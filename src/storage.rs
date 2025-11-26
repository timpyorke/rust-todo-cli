use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{Context, Result};

use crate::models::task::Task;

pub fn load_tasks(path: &str) -> Result<Vec<Task>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path).context("Failed to open todo file")?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).context("Failed to parse todo file")?;
    Ok(tasks)
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .context("Failed to open todo file for writing")?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).context("Failed to write todo file")?;
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

        println!("{} {} {}", format!("{:>3}.", t.id).cyan(), status, t.text);
    }

    // Summary line
    let done_count = tasks.iter().filter(|t| t.done).count();
    let pending_count = tasks.len() - done_count;
    println!(
        "\n{}  {}",
        format!("Done: {done_count}").green(),
        format!("Pending: {pending_count}").yellow()
    );
}
