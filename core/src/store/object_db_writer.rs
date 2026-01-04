use crate::compress::deflate;
use crate::object::Oid;
use crate::store::GitPaths;
use std::fs::{File, create_dir_all};
use std::io::Write;

pub struct ObjectDbWriter {
    paths: GitPaths,
}

impl ObjectDbWriter {
    pub fn new(paths: GitPaths) -> Self {
        Self { paths }
    }
}

impl Write for ObjectDbWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let oid = Oid::from_data(buf);
        let hash = oid.to_string();
        println!("oid {}", hash);
        let (folder_name, file_name) = hash.split_at(2);
        let folder = self.paths.object_dir.join(&folder_name);

        println!("write {:?}", folder);

        create_dir_all(&folder)?;

        let mut file = File::create(folder.join(file_name))?;
        file.write_all(&deflate(buf)?)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
