use std::path::{Path, PathBuf};

const DEFAULT_GITRUST_FOLDER: &str = ".gitrust";
const OBJECTS_FOLDER_NAME: &str = "objects";
const REF_FOLDER_NAME: &str = "refs";
const HEAD_FILE_NAME: &str = "HEAD";
const INDEX_FILE_NAME: &str = "index";

#[derive(Clone)]
pub struct GitPaths {
    pub worktree: PathBuf,
    pub git_dir: PathBuf,
    pub ref_dir: PathBuf,
    pub object_dir: PathBuf,
    pub head_file: PathBuf,
    pub index_file: PathBuf,
}

impl GitPaths {
    pub fn new(worktree: &Path) -> Self {
        let git_dir = worktree.join(DEFAULT_GITRUST_FOLDER);
        Self {
            worktree: worktree.to_owned(),
            git_dir: worktree.join(DEFAULT_GITRUST_FOLDER),
            ref_dir: git_dir.join(REF_FOLDER_NAME),
            object_dir: git_dir.join(OBJECTS_FOLDER_NAME),
            head_file: git_dir.join(HEAD_FILE_NAME),
            index_file: git_dir.join(INDEX_FILE_NAME),
        }
    }
}
