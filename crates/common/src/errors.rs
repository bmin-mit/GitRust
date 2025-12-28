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
pub enum AppError {
    #[error(transparent)]
    CommandNotFound(#[from] CommandNotFound),

    #[error(transparent)]
    GitObject(#[from] GitObjectIsNotBlobErr),
}