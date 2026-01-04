use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct CommandNotFound {}

impl Display for CommandNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandNotFound")
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    CommandNotFound(#[from] CommandNotFound),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
