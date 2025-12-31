use thiserror::Error;
use std::fmt::{Display, Formatter};

#[derive(Error, Debug)]
pub struct CommandNotFound {}

impl Display for CommandNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandNotFound")
    }
}

#[derive(Error, Debug)]
pub struct GitObjectIsNotBlobErr;

impl Display for GitObjectIsNotBlobErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitObjectIsNotBlobErr")
    }
}

#[derive(Error, Debug)]
pub struct GitRepoAlreadyInitialized {}

impl Display for GitRepoAlreadyInitialized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitRepoAlreadyInitialized")
    }
}

#[derive(Error, Debug)]
pub struct GitRepoNotInitialized {}

impl Display for GitRepoNotInitialized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitRepoNotInitialized")
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    CommandNotFound(#[from] CommandNotFound),

    #[error(transparent)]
    GitObject(#[from] GitObjectIsNotBlobErr),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    GitRepoNotInitialized(#[from] GitRepoNotInitialized),
}