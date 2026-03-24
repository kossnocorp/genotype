use std::fs::read_to_string;

use miette::Result;
use sha2::{Digest, Sha256};

use crate::{error::GTWError, path::GTWPath};

pub struct GTWFileSource {
    hash: String,
    content: String,
}

impl GTWFileSource {
    pub fn read(path: &GTWPath) -> Result<GTWFileSource> {
        let content =
            read_to_string(path.as_path()).map_err(|_| GTWError::ReadSource(path.into()))?;

        Ok(GTWFileSource {
            hash: Self::hash(&content),
            content,
        })
    }

    pub fn hash(content: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn same_hash(&self, source: &GTWFileSource) -> bool {
        self.hash == source.hash
    }
}
