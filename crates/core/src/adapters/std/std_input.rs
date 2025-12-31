use crate::ports::TextInput;
use std::io::{Read, Result, stdin};

pub struct StdInput {}

impl StdInput {
    pub fn new() -> Self {
        StdInput {}
    }
}

impl TextInput for StdInput {}

impl Read for StdInput {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        stdin().read(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        stdin().read_to_string(buf)
    }
}
