use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SqliteBackupError {
    SourceFileError(String),
    NoSourceFileError,
}

impl Error for SqliteBackupError {}

impl Display for SqliteBackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqliteBackupError::SourceFileError(source) => {
                write!(f, "Source file error: {}", source)
            }
            SqliteBackupError::NoSourceFileError => write!(f, "No source path provided."),
        }
    }
}
