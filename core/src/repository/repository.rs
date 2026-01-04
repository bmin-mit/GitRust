use crate::repository::repository_fs::RepositoryFs;
use crate::store::{GitPaths, ObjectDb};
use std::path::Path;

pub struct Repository {
    odb: ObjectDb,
}

impl Repository {
    pub fn open(path: &Path) -> Self {
        Self {
            odb: ObjectDb::new(path),
        }
    }

    pub fn init(path: &Path) -> std::io::Result<Self> {
        let git_paths = GitPaths::new(path);
        RepositoryFs::init(&git_paths)?;

        Ok(Repository::open(path))
    }

    pub fn odb(&mut self) -> &mut ObjectDb {
        &mut self.odb
    }
}
