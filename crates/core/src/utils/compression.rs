use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

pub fn deflate(data: Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    encoder.finish()
}
