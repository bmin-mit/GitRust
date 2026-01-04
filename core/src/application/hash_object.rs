use std::fmt::Display;
use std::fs::canonicalize;
use std::io::{Error, Read, stdin};

use crate::object::{Object, ObjectKind, Oid};
use crate::repository::Repository;
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
    git_obj: Option<Object>,
    repository: Repository,
}

impl HashObjectCommand {
    pub fn new(args: HashObjectArgs) -> Result<HashObjectCommand, std::io::Error> {
        Ok(Self {
            args,
            data: None,
            git_obj: None,
            repository: Repository::open(&canonicalize(".")?),
        })
    }

    pub fn run(&mut self) -> Result<(), HashObjectCommandError> {
        self.get_input_data()?;
        self.init_git_obj()?;

        let hash_result = self.hash_git_obj()?;

        if self.args.write {
            self.write_to_repo()?;
        }

        println!("{hash_result}");

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
            self.git_obj = Some(Object::new(ObjectKind::Blob, &data));
            Ok(())
        } else {
            Err(NoInput)
        }
    }

    fn hash_git_obj(&self) -> Result<Oid, HashObjectCommandError> {
        if let Some(obj) = &self.git_obj {
            let hash_result = obj.id();
            Ok(hash_result)
        } else {
            Err(NoInput {})?
        }
    }

    fn write_to_repo(&mut self) -> Result<(), HashObjectCommandError> {
        if let Some(obj) = &self.git_obj {
            self.repository.odb().write(obj)?;
            Ok(())
        } else {
            Err(NoInput {})?
        }
    }
}
