use anyhow::{bail, Result};

use crate::errors::SqliteBackupError;

pub struct Argument {
    pub source_path: String,
}

impl Argument {
    pub fn build(args: &[String]) -> Result<Self> {
        if let Some(source_path) = args.get(1) {
            let argument = Self {
                source_path: source_path.clone(),
            };
            Ok(argument)
        } else {
            bail!(SqliteBackupError::NoSourceFileError);
        }
    }
}
