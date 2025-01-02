use crate::types::{GitObject, ObjectType};
use anyhow::Context;
use std::fs::File;
use std::io::Read;

pub fn hash_object(file_path: &str) -> anyhow::Result<()> {
    let mut file = File::open(file_path).context("Failed to read file")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    let go = GitObject {
        object_type: ObjectType::Blob,
        object_size: buf.len(),
        object_content: buf,
    };
    let hash = go.hash();
    // Note, this works since the hash will only ever contain ASCII characters
    let hash_folder_name = &hash[..2];
    let hash_file_name = &hash[2..];
    let path = format!(".git/objects/{}/{}", hash_folder_name, hash_file_name);

    go.to_file(&path)?;
    println!("{}", hash);
    Ok(())
}
