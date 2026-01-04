use crate::object::Object;
use crate::store::object_db_writer::ObjectDbWriter;
use crate::store::paths::GitPaths;
use std::io::Write;
use std::path::Path;

pub struct ObjectDb {
    writer: ObjectDbWriter,
}

impl ObjectDb {
    pub fn new(path: &Path) -> Self {
        Self {
            writer: ObjectDbWriter::new(GitPaths::new(path)),
        }
    }

    pub fn write(&mut self, obj: &Object) -> std::io::Result<()> {
        println!("write");
        self.writer.write(obj.store_content())?;
        Ok(())
    }
}
