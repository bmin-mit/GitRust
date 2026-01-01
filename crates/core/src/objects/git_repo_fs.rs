use crate::objects::{GitObject, DEFAULT_GITRUST_FOLDER};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::utils::hash_to_string;

pub struct GitRepoFs {
    pub root_path: PathBuf,
    pub git_dir_path: PathBuf,
}

const OBJECTS_FOLDER_NAME: &str = "objects";

impl GitRepoFs {
    pub fn new(root_path: &Path) -> Self {
        Self {
            root_path: root_path.to_owned(),
            git_dir_path: root_path.join(DEFAULT_GITRUST_FOLDER),
        }
    }

    pub fn init_repo_on_fs(&self) -> std::io::Result<()> {
        create_dir_all(&self.git_dir_path)
    }

    pub fn write_obj_to_db(&self, obj: &GitObject) -> std::io::Result<()> {
        let hash = obj.hash();
        let hashed_string = hash_to_string(&hash);

        let folder_name = hashed_string[..2].to_owned();
        let hash_folder = self.object_folder().join(folder_name);
        create_dir_all(&hash_folder)?;

        self.write_hash_to_folder(&hashed_string, obj, hash_folder)?;

        Ok(())
    }

    fn write_hash_to_folder(&self, hash: &str, obj: &GitObject, folder: PathBuf) -> std::io::Result<()> {
        let file_name = hash[2..].to_owned();
        let deflated_content = obj.deflate()?;
        let mut file = File::create(folder.join(file_name))?;
        file.write_all(&deflated_content)?;
        Ok(())
    }

    fn object_folder(&self) -> PathBuf {
        self.git_dir_path.join(OBJECTS_FOLDER_NAME)
    }
}