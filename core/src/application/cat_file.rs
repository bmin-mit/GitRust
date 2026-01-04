use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
enum CatFileCommandError {}

pub struct CatFileCommand {}

impl CatFileCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run() {}
}
