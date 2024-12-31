#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CatFileError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("Invalid object")]
    InvalidObject,
    #[error("Not a blob")]
    NotABlob,
}

#[derive(Debug, thiserror::Error)]
pub enum HashObjectError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("Invalid object")]
    InvalidObject,
}

#[derive(Debug, thiserror::Error)]
pub enum LSTreeError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("Invalid object")]
    InvalidObject,
}

#[derive(Debug, thiserror::Error)]
pub enum ObjectError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("Invalid object")]
    InvalidObject,
    #[error("Mutilple objects found for hash {0}")]
    MultipleObjectsFound(String),
    #[error("Invalid hash length. Hash must be between 2 and 40 characters: got {0}.")]
    InvalidHashLength(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Cat File Error: {0}")]
    CatFileError(#[from] CatFileError),
    #[error("Init Error: {0}")]
    InitError(#[from] InitError),
    #[error("Hash Object Error: {0}")]
    HashObjectError(#[from] HashObjectError),
    #[error("Object Error: {0}")]
    ObjectError(#[from] ObjectError),
}
