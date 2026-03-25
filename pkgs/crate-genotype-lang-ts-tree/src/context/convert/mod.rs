use crate::prelude::internal::*;
use std::collections::HashMap;

mod doc;
pub mod hoisting;
mod resolve;

#[derive(Debug, Clone, PartialEq)]
pub struct TsConvertContext {
    resolve: TsConvertResolve,
    hoisted: Vec<TsDefinition>,
    doc: Option<TsDoc>,
    dependencies_config: HashMap<String, String>,
}

impl TsConvertContext {
    pub fn new(resolve: TsConvertResolve, dependencies_config: HashMap<String, String>) -> Self {
        Self {
            resolve,
            hoisted: vec![],
            doc: None,
            dependencies_config,
        }
    }
}

impl Default for TsConvertContext {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}
