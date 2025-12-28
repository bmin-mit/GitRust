use std;
use std::io::Read;

pub fn read_from_stdin() -> Vec<u8> {
    let mut result = Vec::new();
    std::io::stdin().read_to_end(&mut result).unwrap();

    result
}