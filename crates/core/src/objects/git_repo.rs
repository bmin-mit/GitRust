use crate::objects::{GitObject, GitRepoFs};
use std::path::Path;

pub const DEFAULT_GITRUST_FOLDER: &str = ".gitrust";

pub struct GitRepo {
    repo_fs: GitRepoFs,
}

impl GitRepo {
    pub fn new(root_path: &Path) -> Self {
        Self { repo_fs: GitRepoFs::new(root_path) }
    }

    pub fn default() -> Self {
        let pwd = Path::new(".");
        GitRepo::new(pwd)
    }

    pub fn is_valid_repo(&self) -> bool {
        self.repo_fs.git_dir_path.exists() && self.repo_fs.git_dir_path.is_dir()
    }

    pub fn create_git_dir(&self) -> std::io::Result<()> {
        self.repo_fs.init_repo_on_fs()
    }

    pub fn write_obj_to_db(&self, obj: &GitObject) -> Result<(), std::io::Error> {
        self.repo_fs.write_obj_to_db(&obj)
    }
}
