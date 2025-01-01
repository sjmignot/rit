use anyhow::Context;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use glob::glob;
use hex;
use sha1::{Digest, Sha1};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

#[derive(Debug)]
pub enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
}
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct GitObject {
    pub object_type: ObjectType,
    pub object_size: usize,
    pub object_content: Vec<u8>,
}

impl GitObject {
    pub fn from_file<T: Read + BufRead>(object_handler: &mut T) -> anyhow::Result<Self> {
        let mut header = vec![];

        object_handler
            .read_until(b'\0', &mut header)
            .context("Failed to read header")?;
        header.truncate(header.len() - 1);
        let header = String::from_utf8(header).context("Failed to parse header")?;

        // print header as bytes
        let (object_type_str, object_size_str) = header
            .split_once(' ')
            .ok_or(anyhow::anyhow!("Failed to split header"))?;

        let object_size = object_size_str
            .trim()
            .parse::<usize>()
            .context("Failed to parse object size")?;

        let object_type = match object_type_str {
            "blob" => ObjectType::Blob,
            "tree" => ObjectType::Tree,
            "commit" => ObjectType::Commit,
            "tag" => ObjectType::Tag,
            _ => anyhow::bail!("Invalid object type"),
        };

        let mut content = vec![];
        object_handler
            .take(object_size as u64)
            .read_to_end(&mut content)
            .context("Failed to read content")?;

        Ok(Self {
            object_type,
            object_size,
            object_content: content,
        })
    }
    pub fn to_file(&self, file_path: &str) -> anyhow::Result<()> {
        let path = std::path::Path::new(&file_path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);
        let mut encoder = ZlibEncoder::new(&mut writer, Compression::default());
        write!(
            encoder,
            "{} {}\x00",
            self.object_type.to_string(),
            self.object_size
        )
        .context("Failed to write object header")?;
        encoder
            .write(self.object_content.as_slice())
            .context("Failed to write object content")?;
        Ok(())
    }

    fn find_hash(object_hash: &str) -> anyhow::Result<impl Read> {
        let object_folder = &object_hash[..2];
        let object_file = &object_hash[2..];

        let file_len = object_hash.len();

        if file_len < 2 || file_len > 40 {
            anyhow::bail!("Invalid hash length: {}", file_len);
        }

        let path = format!(".git/objects/{}/{}", object_folder, object_file);

        if file_len == 40 {
            File::open(&path).context("Failed to open file: {path}")
        } else {
            let mut hash_match = glob(&format!("{}{}", path, "*")).expect("failed to get paths");

            let path = hash_match
                .next()
                .context("No matches found")?
                .context("Failed to get the path")?;

            if hash_match.next().is_some() {
                anyhow::bail!("Multiple files found for hash {}", object_hash);
            }
            let file = File::open(&path).context("Failed to open file")?;
            Ok(file)
        }
    }

    pub fn from_hash(object_hash: &str) -> anyhow::Result<GitObject> {
        let file = Self::find_hash(object_hash)?;
        let mut zlib = BufReader::new(ZlibDecoder::new(file));
        Self::from_file(&mut zlib)
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(format!("{} {}\x00", self.object_type, self.object_size));
        hasher.update(&self.object_content);
        hex::encode(hasher.finalize())
    }
    pub fn pretty_print(&self) -> anyhow::Result<()> {
        match self.object_type {
            ObjectType::Blob => {
                print!(
                    "{}",
                    String::from_utf8(self.object_content.clone())
                        .context("Failed to parse content")?
                );
            }
            ObjectType::Tree => {
                let mut content_iterator = self.object_content.clone().into_iter();
                while content_iterator.len() > 0 {
                    let mode: Vec<u8> = content_iterator
                        .by_ref()
                        .take_while(|&x| x != 32)
                        .collect::<Vec<_>>();
                    let name: Vec<u8> = content_iterator
                        .by_ref()
                        .take_while(|&x| x != 0)
                        .collect::<Vec<_>>();
                    let sha: Vec<u8> = content_iterator.by_ref().take(20).collect::<Vec<_>>();
                    let object_type = match mode.as_slice() {
                        b"40000" => "tree",
                        _ => "blob",
                    };
                    println!(
                        "{:<6} {} {} {}",
                        String::from_utf8(mode)?,
                        object_type,
                        hex::encode(sha),
                        String::from_utf8(name)?,
                    );
                }
            }
            _ => {
                anyhow::bail!("Not implemented");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sha1::digest::typenum::assert_type;

    use super::*;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn test_cat_file() {
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(b"blob 5\x00hello").unwrap();
        tmp_file.seek(SeekFrom::Start(0)).unwrap();
        let result = GitObject::from_file(&mut tmp_file);
        assert!(result.is_ok());
        let object = result.unwrap();
        assert_eq!(object.object_type, ObjectType::Blob);
        assert!(object.object_size == 5);
        assert!(object.object_content == b"hello");
    }
}
