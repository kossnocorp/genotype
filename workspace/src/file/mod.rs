use genotype_config::GTConfig;
use indexmap::IndexMap;
use miette::Result;
use std::sync::{Arc, Mutex};

use crate::path::GTWPath;

pub mod source;
pub use source::*;

pub mod payload;
pub use payload::*;

pub mod kind;
pub use kind::*;

pub type GTWFiles = Arc<Mutex<IndexMap<GTWPath, GTWFile>>>;

pub struct GTWFile {
    source: GTWFileSource,
    payload: GTWFilePayload,
}

impl GTWFile {
    pub fn load(path: &GTWPath, source: GTWFileSource) -> Result<Self> {
        match GTWFileKind::detect(path)? {
            GTWFileKind::Config => {}

            GTWFileKind::Module => {}
        }

        Ok(GTWFile {
            source,
            payload: GTWFilePayload::Config(GTConfig::default()),
        })
    }

    pub fn same_hash(&self, source: &GTWFileSource) -> bool {
        self.source.same_hash(source)
    }
}
