use std::collections::{HashMap, HashSet};

use genotype_parser::{GTDefinitionId, GTSpan};

#[derive(Debug, PartialEq, Clone)]
pub struct RSProjectModuleResolve {
    pub references: HashMap<GTDefinitionId, HashSet<GTSpan>>,
}
