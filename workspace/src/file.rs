use std::sync::{Arc, Mutex};

use genotype_config::GTConfig;
use genotype_parser::GTModule;
use indexmap::IndexMap;

use crate::path::GTWPath;

pub type GTWFiles = Arc<Mutex<IndexMap<GTWPath, GTWFile>>>;

pub enum GTWFile {
    Loading,
    Valid {
        hash: String,
        payload: GTWFilePayload,
    },
    Invalid {
        hash: String,
        content: GTWFilePayload,
    },
}

impl GTWFile {
    pub fn read_source(path: &GTWPath) -> GTWFileSource {
        // [TODO]
    }
}

pub struct GTWFileSource {
    pub hash: String,
    pub content: String,
}

pub enum GTWFilePayload {
    Config(GTConfig),
    Module(GTModule),
}
