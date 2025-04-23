use crate::prelude::internal::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct TSConvertResolve {
    pub paths: HashMap<GTPath, GTPath>,
    pub globs: HashMap<GTPath, String>,
    pub identifiers: HashMap<GTIdentifier, GTIdentifier>,
}

impl TSConvertResolve {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
        }
    }
}

impl Default for TSConvertResolve {
    fn default() -> Self {
        Self::new()
    }
}
