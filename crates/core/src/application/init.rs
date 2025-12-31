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
}

impl InitCommand {
    pub fn new(args: InitArgs) -> Self {
        Self { args }
    }

    pub fn init(&self) -> Result<(), InitCommandError> {
        let pwd = current_dir().unwrap();

        let repo = GitRepo::new(&pwd);
        if repo.is_valid_repo() {
            Err(GitRepoAlreadyInitialized {})?
        }

        Self::create_git_dir(repo)?;

        Ok(())
    }

    fn create_git_dir(repo: GitRepo) -> io::Result<()> {
        create_dir_all(repo.git_dir_path)
    }
}
