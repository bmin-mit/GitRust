use gitrust_common::cli::args::HashObjectArgs;
use gitrust_common::errors::GitObjectIsNotBlobErr;
use gitrust_common::types::GitObject;
use crate::io;

pub fn hash_object(args: HashObjectArgs) -> Result<(), GitObjectIsNotBlobErr> {
    let mut data = Vec::new();
    if args.stdin {
        data = io::read_from_stdin();
    }

    let object = GitObject::from_array(&data);

    let hash_result = object.hash()?;

    let hash_result = hash_result
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{hash_result}");

    Ok(())
}