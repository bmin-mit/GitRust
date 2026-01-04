use crate::repository::Repository;
use gitrust_cli_parser::args::InitArgs;
use std::fs::canonicalize;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum InitCommandError {
    IoError(#[from] std::io::Error),
}

pub struct InitCommand {
    args: InitArgs,
}

impl InitCommand {
    pub fn new(args: InitArgs) -> Self {
        Self { args }
    }

    pub fn run(&self) -> Result<(), InitCommandError> {
        Repository::init(&canonicalize(".")?)?;
        Ok(())
    }
}
