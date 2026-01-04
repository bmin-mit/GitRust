use crate::object::{ObjectKind, Oid};

#[derive(Clone, PartialEq, Eq)]
pub struct Object {
    id: Oid,
    kind: ObjectKind,
    data: Vec<u8>,
    header: Vec<u8>,
    store_content: Vec<u8>,
}

impl Object {
    pub fn new(kind: ObjectKind, data: &[u8]) -> Self {
        let header = Self::init_header(kind, &data);
        let store_content = Self::init_store_content(kind, &data);
        let id = Oid::from_data(&store_content);
        Object {
            id,
            header,
            store_content,
            kind,
            data: data.to_owned(),
        }
    }

    fn init_header(kind: ObjectKind, data: &[u8]) -> Vec<u8> {
        let mut header = Vec::<u8>::new();
        let size = data.len();

        let header_tag = match kind {
            ObjectKind::Blob => String::from("blob "),
            ObjectKind::Tree => String::from("tree "),
            ObjectKind::Commit => String::from("commit "),
            ObjectKind::Tag => String::from("tag "),
        };

        header.extend_from_slice(header_tag.as_bytes());
        header.extend_from_slice(size.to_string().as_bytes());
        header.push(b'\0');

        header
    }

    fn init_store_content(kind: ObjectKind, data: &[u8]) -> Vec<u8> {
        let mut store_content = Self::init_header(kind, data);
        store_content.extend_from_slice(data);
        store_content
    }

    pub fn id(&self) -> Oid {
        self.id.clone()
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn store_content(&self) -> &[u8] {
        &self.store_content
    }
}
