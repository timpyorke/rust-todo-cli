use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{Context, Result};

use crate::models::task::Task;
use crate::constants::*;

pub fn load_tasks(path: &str) -> Result<Vec<Task>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path).context(ERR_OPEN_FILE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).context(ERR_PARSE_FILE)?;
    Ok(tasks)
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
        println!("{}", MSG_NO_TASKS_SHOW.green());
        return;
    }

    println!("{}", MSG_YOUR_TASKS.bold());
    for t in tasks {
        let status = if t.done {
            STATUS_DONE.green()
        } else {
            STATUS_PENDING.yellow()
        };

        println!("{} {} {}", format!("{:>3}.", t.id).cyan(), status, t.text);
    }

    // Summary line
    let done_count = tasks.iter().filter(|t| t.done).count();
    let pending_count = tasks.len() - done_count;
    println!(
        "\n{}  {}",
        format!("{} {done_count}", LABEL_DONE).green(),
        format!("{} {pending_count}", LABEL_PENDING).yellow()
    );
}


#[cfg(test)]
mod tests;
