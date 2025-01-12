use genotype_config::GTConfig;
use genotype_parser::GTModule;
use indexmap::IndexMap;
use miette::Result;
use sha2::{Digest, Sha256};
use std::{
    fs::read_to_string,
    sync::{Arc, Mutex},
};

use crate::{
    error::GTWError,
    path::{GTWPath, GTWPathKind},
};

pub type GTWFiles = Arc<Mutex<IndexMap<GTWPath, GTWFile>>>;

pub struct GTWFile {
    hash: String,
    payload: GTWFilePayload,
}

impl GTWFile {
    pub fn load(path: &GTWPath, source: &GTWFileSource) -> Result<Self> {
        match path.kind() {
            GTWPathKind::Config => {}

            GTWPathKind::Module => {}
        }

        Ok(GTWFile {
            hash: String::new(),
            payload: GTWFilePayload::Config(GTConfig::default()),
        })
    }

    pub fn same_hash(&self, source: &GTWFileSource) -> bool {
        source.same_hash(&self.hash)
    }
}

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

    pub fn same_hash(&self, hash: &String) -> bool {
        self.hash == *hash
    }
}

pub enum GTWFilePayload {
    Config(GTConfig),
    Module(GTModule),
}
