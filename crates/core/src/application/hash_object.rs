use std::fmt::Display;
use std::io::Read;

use crate::adapters::StdInput;
use crate::errors::GitObjectIsNotBlobErr;
use crate::objects::GitObject;
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
    ObjectIsNotBlob(#[from] GitObjectIsNotBlobErr),
    NoInput(#[from] NoInput),
    IoError(#[from] std::io::Error),
}

pub struct HashObjectCommand {
    args: HashObjectArgs,
    data: Option<Vec<u8>>,
    git_obj: Option<GitObject>,
}

impl HashObjectCommand {
    pub fn new(args: HashObjectArgs) -> Self {
        Self {
            args,
            data: None,
            git_obj: None,
        }
    }

    pub fn hash_object(&mut self) -> Result<(), HashObjectCommandError> {
        self.get_input_data()?;
        self.init_git_obj()?;

        let hash_result = self.hash_git_obj()?;
        let formatted_hash = self.format_hash(&hash_result);

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
        StdInput::new()
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
            let hash_result = obj.hash()?;
            Ok(hash_result)
        } else {
            Err(NoInput {})?
        }
    }

    fn format_hash(&self, hash: &[u8]) -> String {
        hash.iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
