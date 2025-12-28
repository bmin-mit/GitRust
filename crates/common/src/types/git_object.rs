use gitrust_core::hashing::hash;
use crate::errors::GitObjectIsNotBlobErr;

pub struct GitBlobObject {
    data: Vec<u8>,
}

pub enum GitObjectType {
    Blob(GitBlobObject),
    Tree,
}

pub struct GitObject {
    pub obj_type: GitObjectType,
}

impl GitObject {
    pub fn from_string(data: &str) -> Self {
        GitObject {
            obj_type: GitObjectType::Blob(
                GitBlobObject {
                    data: data.as_bytes().to_vec()
                }
            )
        }
    }

    pub fn from_array(data: &[u8]) -> Self {
        GitObject {
            obj_type: GitObjectType::Blob(
                GitBlobObject {
                    data: data.to_vec()
                }
            )
        }
    }

    pub fn hash(&self) -> Result<Vec<u8>, GitObjectIsNotBlobErr> {
        let header = self.header()?;
        let data = self.data()?;

        let mut store = header;
        store.extend_from_slice(&data);

        Ok(hash(&store))
    }

    fn data(&self) -> Result<&Vec<u8>, GitObjectIsNotBlobErr> {
        if let GitObjectType::Blob(blob) = &self.obj_type {
            let GitBlobObject { data } = blob;
            return Ok(data);
        }
        Err(GitObjectIsNotBlobErr)
    }

    fn header(&self) -> Result<Vec<u8>, GitObjectIsNotBlobErr> {
        let mut header = Vec::<u8>::new();
        let data = self.data()?;
        let size = data.len();

        header.extend_from_slice(b"blob ");
        header.extend_from_slice(size.to_string().as_bytes());
        header.push(b'\0');

        Ok(header)
    }
}
