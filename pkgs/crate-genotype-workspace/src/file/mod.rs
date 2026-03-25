use genotype_config::GtConfig;
use indexmap::IndexMap;
use miette::Result;
use std::sync::{Arc, Mutex};

use crate::path::GtwPath;

pub mod source;
pub use source::*;

pub mod payload;
pub use payload::*;

pub mod kind;
pub use kind::*;

pub type GtwFiles = Arc<Mutex<IndexMap<GtwPath, GtwFile>>>;

pub struct GtwFile {
    source: GtwFileSource,
    payload: GtwFilePayload,
}

impl GtwFile {
    pub fn load(path: &GtwPath, source: GtwFileSource) -> Result<Self> {
        match GtwFileKind::detect(path)? {
            GtwFileKind::Config => {}

            GtwFileKind::Module => {}
        }

        Ok(GtwFile {
            source,
            payload: GtwFilePayload::Config(GtConfig::default()),
        })
    }

    pub fn same_hash(&self, source: &GtwFileSource) -> bool {
        self.source.same_hash(source)
    }
}
