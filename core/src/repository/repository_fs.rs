use crate::store::GitPaths;
use std::fs::{File, create_dir_all};
use std::io::Write;

pub struct RepositoryFs;

impl RepositoryFs {
    pub fn init(paths: &GitPaths) -> std::io::Result<()> {
        create_dir_all(&paths.git_dir)?;

        Self::init_head_file(paths)?;
        Self::init_object_folder(paths)?;
        Self::init_ref_folder(paths)?;

        Ok(())
    }

    fn init_head_file(paths: &GitPaths) -> std::io::Result<()> {
        let mut file = File::create(&paths.head_file)?;
        const DEFAULT_CONTENT: &str = "ref: refs/heads/main";
        file.write_all(DEFAULT_CONTENT.as_bytes())?;

        Ok(())
    }

    fn init_object_folder(paths: &GitPaths) -> std::io::Result<()> {
        let obj_folder = &paths.object_dir;

        create_dir_all(&obj_folder)?;
        create_dir_all(&obj_folder.join("info"))?;
        create_dir_all(&obj_folder.join("pack"))?;

        Ok(())
    }

    fn init_ref_folder(paths: &GitPaths) -> std::io::Result<()> {
        let ref_folder = &paths.ref_dir;

        create_dir_all(&ref_folder)?;
        create_dir_all(&ref_folder.join("heads"))?;
        create_dir_all(&ref_folder.join("tags"))?;

        Ok(())
    }
}
