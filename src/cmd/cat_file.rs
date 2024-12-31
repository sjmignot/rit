use crate::types::GitObject;
use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::BufReader;

pub fn cat_file(blob_sha: &str) -> anyhow::Result<()> {
    let path = format!(".git/objects/{}/{}", &blob_sha[..2], &blob_sha[2..]);
    let file = File::open(&path).context("Failed to open file")?;
    let mut zlib = BufReader::new(ZlibDecoder::new(file));
    let object = GitObject::from_file(&mut zlib).context("Failed to read object")?;
    object.pretty_print()?;
    Ok(())
}
