use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub done: bool,
    pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    #[default]
    Normal,
    High,
}
