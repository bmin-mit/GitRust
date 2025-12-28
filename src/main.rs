use gitrust_cli_parser::CliParser;
use gitrust_io::trigger_command;

fn main() -> anyhow::Result<()> {
    let cli = CliParser::parse();

    trigger_command(cli.command)?;

    Ok(())
}
