use std::collections::HashSet;

use super::{GTIdentifier, GTPath};

#[derive(Debug, PartialEq, Clone)]
pub struct GTResolve {
    pub deps: HashSet<GTPath>,
    pub exports: Vec<GTIdentifier>,
    pub references: HashSet<GTIdentifier>,
}
