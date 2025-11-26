use clap::Parser;

use super::commands::Commands;

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple Rust CLI todo app", long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
