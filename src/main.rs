use gitrust_cli_parser::{CliCommands, CliParser};
use gitrust_core::{application::*, errors::CommandNotFound};

fn main() -> anyhow::Result<()> {
    let cli = CliParser::parse();

    match cli.command {
        Some(CliCommands::HashObject(args)) => HashObjectCommand::new(args)?.run()?,
        Some(CliCommands::Init(args)) => InitCommand::new(args).run()?,
        _ => Err(CommandNotFound {})?,
    }

    Ok(())
}
