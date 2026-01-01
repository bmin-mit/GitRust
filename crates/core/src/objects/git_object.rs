use crate::utils::{deflate, hash};

pub struct GitBlobObject {
    data: Vec<u8>,
}

pub enum GitObjectType {
    Blob(GitBlobObject),
    Tree,
    Commit,
    Tag,
}

pub struct GitObject {
    pub obj_type: GitObjectType,
}

impl GitObject {
    pub fn from_string(data: &str) -> Self {
        GitObject {
            obj_type: GitObjectType::Blob(GitBlobObject {
                data: data.as_bytes().to_vec(),
            }),
        }
    }

    pub fn from_array(data: &[u8]) -> Self {
        GitObject {
            obj_type: GitObjectType::Blob(GitBlobObject {
                data: data.to_vec(),
            }),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        hash(&self.store_content())
    }

    pub fn deflate(&self) -> std::io::Result<Vec<u8>> {
        deflate(self.store_content())
    }

    fn data(&self) -> Vec<u8> {
        if let GitObjectType::Blob(blob) = &self.obj_type {
            let GitBlobObject { data } = blob;
            data.to_owned()
        } else {
            Vec::<u8>::new()
        }
    }

    fn header(&self) -> Vec<u8> {
        let mut header = Vec::<u8>::new();
        let data = self.data();
        let size = data.len();

        header.extend_from_slice(b"blob ");
        header.extend_from_slice(size.to_string().as_bytes());
        header.push(b'\0');

        header
    }

    fn store_content(&self) -> Vec<u8> {
        let header = self.header();
        let data = self.data();

        let mut store = header;
        store.extend_from_slice(&data);

        store
    }
}
