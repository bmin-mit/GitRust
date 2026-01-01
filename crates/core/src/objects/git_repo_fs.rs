use crate::objects::{DEFAULT_GITRUST_FOLDER, GitObject};
use crate::utils::hash_to_string;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct GitRepoFs {
    pub root_path: PathBuf,
    pub git_dir_path: PathBuf,
}

const OBJECTS_FOLDER_NAME: &str = "objects";
const REF_FOLDER_NAME: &str = "refs";

impl GitRepoFs {
    pub fn new(root_path: &Path) -> Self {
        Self {
            root_path: root_path.to_owned(),
            git_dir_path: root_path.join(DEFAULT_GITRUST_FOLDER),
        }
    }

    pub fn init_repo_on_fs(&self) -> std::io::Result<()> {
        create_dir_all(&self.git_dir_path)?;

        self.init_head_file()?;
        self.init_object_folder()?;
        self.init_ref_folder()?;

        Ok(())
    }

    fn init_head_file(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.head_file())?;
        const DEFAULT_CONTENT: &str = "ref: refs/heads/main";
        file.write_all(DEFAULT_CONTENT.as_bytes())?;

        Ok(())
    }

    fn init_object_folder(&self) -> std::io::Result<()> {
        let obj_folder = self.object_folder();

        create_dir_all(&obj_folder)?;
        create_dir_all(&obj_folder.join("info"))?;
        create_dir_all(&obj_folder.join("pack"))?;

        Ok(())
    }

    fn init_ref_folder(&self) -> std::io::Result<()> {
        let ref_folder = self.ref_folder();

        create_dir_all(&ref_folder)?;
        create_dir_all(&ref_folder.join("heads"))?;
        create_dir_all(&ref_folder.join("tags"))?;

        Ok(())
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

    fn write_hash_to_folder(
        &self,
        hash: &str,
        obj: &GitObject,
        folder: PathBuf,
    ) -> std::io::Result<()> {
        let file_name = hash[2..].to_owned();
        let deflated_content = obj.deflate()?;
        let mut file = File::create(folder.join(file_name))?;
        file.write_all(&deflated_content)?;
        Ok(())
    }

    fn ref_folder(&self) -> PathBuf {
        self.git_dir_path.join(REF_FOLDER_NAME)
    }

    fn object_folder(&self) -> PathBuf {
        self.git_dir_path.join(OBJECTS_FOLDER_NAME)
    }

    fn head_file(&self) -> PathBuf {
        self.git_dir_path.join("HEAD")
    }
}
