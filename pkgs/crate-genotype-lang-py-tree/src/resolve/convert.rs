use crate::prelude::internal::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
#[derive(Default)]
pub struct PyConvertResolve {
    pub paths: HashMap<GtPath, GtPath>,
    pub globs: HashMap<GtPath, String>,
    pub identifiers: HashMap<GtIdentifier, GtIdentifier>,
    pub imported: HashSet<GtIdentifier>,
}

