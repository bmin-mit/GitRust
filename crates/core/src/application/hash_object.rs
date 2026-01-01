use std::fmt::Display;
use std::io::{Read, stdin};

use crate::objects::{GitObject, GitRepo};
use crate::utils::hash_to_string;
use gitrust_cli_parser::args::HashObjectArgs;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct NoInput;

impl Display for NoInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoInput")
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum HashObjectCommandError {
    NoInput(#[from] NoInput),
    IoError(#[from] std::io::Error),
}

pub struct HashObjectCommand {
    args: HashObjectArgs,
    data: Option<Vec<u8>>,
    git_obj: Option<GitObject>,
    repo: GitRepo,
}

impl HashObjectCommand {
    pub fn new(args: HashObjectArgs) -> Self {
        Self {
            args,
            data: None,
            git_obj: None,
            repo: GitRepo::default(),
        }
    }

    pub fn hash_object(&mut self) -> Result<(), HashObjectCommandError> {
        self.get_input_data()?;
        self.init_git_obj()?;

        let hash_result = self.hash_git_obj()?;
        let formatted_hash = self.format_hash(&hash_result);

        if (self.args.write) {
            self.write_to_repo()?;
        }

        println!("{formatted_hash}");

        Ok(())
    }

    fn get_input_data(&mut self) -> Result<(), std::io::Error> {
        let input_data = {
            let mut input_source = self.get_input_source();
            let mut input_data = Vec::new();
            input_source.read_to_end(&mut input_data)?;
            input_data
        };

        self.data = Some(input_data.to_owned());
        Ok(())
    }

    fn get_input_source(&self) -> impl Read {
        stdin()
    }

    fn init_git_obj(&mut self) -> Result<(), NoInput> {
        if let Some(data) = &self.data {
            self.git_obj = Some(GitObject::from_array(&data));
            Ok(())
        } else {
            Err(NoInput)
        }
    }

    fn hash_git_obj(&self) -> Result<Vec<u8>, HashObjectCommandError> {
        if let Some(obj) = &self.git_obj {
            let hash_result = obj.hash();
            Ok(hash_result)
        } else {
            Err(NoInput {})?
        }
    }

    fn format_hash(&self, hash: &[u8]) -> String {
        hash_to_string(hash)
    }

    fn write_to_repo(&self) -> std::io::Result<()> {
        self.repo.write_obj_to_db(&self.git_obj.as_ref().unwrap())
    }
}
