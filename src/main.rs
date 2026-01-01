use gitrust_cli_parser::{CliCommands, CliParser};
use gitrust_core::{
    application::*,
    errors::CommandNotFound,
};

fn main() -> anyhow::Result<()> {
    let cli = CliParser::parse();

    match cli.command {
        Some(CliCommands::HashObject(args)) => HashObjectCommand::new(args).hash_object()?,
        Some(CliCommands::Init(args)) => InitCommand::new(args).init()?,
        _ => Err(CommandNotFound {})?,
    }

    Ok(())
}
