use std::fs;
use std::path::{Path, PathBuf};
use crate::errors::GitRepoNotInitialized;

pub const DEFAULT_GITRUST_FOLDER: &str = ".gitrust";

pub struct GitRepo {
    pub root_path: PathBuf,
    pub git_dir_path: PathBuf,
}

impl GitRepo {
    pub fn new(root_path: &Path) -> Self {
        Self {
            root_path: root_path.to_owned(),
            git_dir_path: root_path.join(DEFAULT_GITRUST_FOLDER),
        }
    }
    
    pub fn is_valid_repo(&self) -> bool {
        self.git_dir_path.exists() && self.git_dir_path.is_dir()
    }
}
