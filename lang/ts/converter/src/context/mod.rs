use std::collections::HashMap;

use genotype_lang_ts_tree::{TSDefinition, TSDoc};
use genotype_parser::GTPath;

use crate::resolve::TSConvertResolve;

pub mod doc;
pub mod hoisting;
pub mod resolve;

pub struct TSConvertContext {
    resolve: TSConvertResolve,
    hoisted: Vec<TSDefinition>,
    doc: Option<TSDoc>,
    dependencies_config: HashMap<String, String>,
}

impl TSConvertContext {
    pub fn new(
        resolve: TSConvertResolve,
        dependencies_config: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            resolve,
            hoisted: vec![],
            doc: None,
            dependencies_config: dependencies_config.unwrap_or_default(),
        }
    }
}

impl Default for TSConvertContext {
    fn default() -> Self {
        Self::new(Default::default(), None)
    }
}
