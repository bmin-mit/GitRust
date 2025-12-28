mod commands;
mod io;

use crate::commands::*;
use gitrust_common::cli::CliCommands;
use gitrust_common::errors::{AppError, CommandNotFound};

pub fn trigger_command(command: Option<CliCommands>) -> Result<(), AppError> {
    match command {
        Some(CliCommands::HashObject(args)) => hash_object(args)?,
        None => Err(CommandNotFound {})?
    }

    Ok(())
}