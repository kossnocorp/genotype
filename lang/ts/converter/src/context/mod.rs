use genotype_lang_ts_tree::{TSDefinition, TSDoc};

use crate::resolve::TSConvertResolve;

pub mod doc;
pub mod hoisting;
pub mod resolve;

pub struct TSConvertContext {
    resolve: TSConvertResolve,
    hoisted: Vec<TSDefinition>,
    doc: Option<TSDoc>,
}

impl TSConvertContext {
    pub fn new(resolve: TSConvertResolve) -> Self {
        Self {
            resolve,
            hoisted: vec![],
            doc: None,
        }
    }
}

impl Default for TSConvertContext {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
