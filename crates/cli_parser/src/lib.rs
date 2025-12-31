pub mod args;
pub mod commands;

use clap::Parser;
pub use crate::commands::CliCommands;

#[derive(Parser)]
#[command(name = "GitRust")]
#[command(version, about = "A Git implementation in Rust", long_about = None)]
pub struct CliParser {
    #[command(subcommand)]
    pub command: Option<CliCommands>,
}

impl CliParser {
    pub fn parse() -> Self {
        <CliParser as Parser>::parse()
    }
}
