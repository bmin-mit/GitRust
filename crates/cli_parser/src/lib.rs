use clap::Parser;
use gitrust_common::cli::CliCommands;

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
