use std::fs::read_to_string;

use miette::Result;
use sha2::{Digest, Sha256};

use crate::{error::GtwError, path::GtwPath};

pub struct GtwFileSource {
    hash: String,
    content: String,
}

impl GtwFileSource {
    pub fn read(path: &GtwPath) -> Result<GtwFileSource> {
        let content =
            read_to_string(path.as_path()).map_err(|_| GtwError::ReadSource(path.into()))?;

        Ok(GtwFileSource {
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

    pub fn same_hash(&self, source: &GtwFileSource) -> bool {
        self.hash == source.hash
    }
}
