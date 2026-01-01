use crate::errors::GitRepoAlreadyInitialized;
use crate::objects::GitRepo;
use gitrust_cli_parser::args::InitArgs;
use std::{env::current_dir, fs::create_dir_all, io};
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum InitCommandError {
    AlreadyInitialized(#[from] GitRepoAlreadyInitialized),
    IoError(#[from] std::io::Error),
}

pub struct InitCommand {
    args: InitArgs,
    repo: GitRepo,
}

impl InitCommand {
    pub fn new(args: InitArgs) -> Self {
        Self {
            args,
            repo: GitRepo::default(),
        }
    }

    pub fn init(&self) -> Result<(), InitCommandError> {
        if self.repo.is_valid_repo() {
            Err(GitRepoAlreadyInitialized {})?
        }

        self.repo.create_git_dir()?;

        Ok(())
    }
}
