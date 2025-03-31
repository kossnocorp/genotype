use std::collections::HashMap;

use genotype_parser::GTDefinitionId;
use genotype_project::GTPModuleDefinitionResolve;

#[derive(Debug, PartialEq, Clone)]
pub struct RSPModuleResolve {
    pub definitions: HashMap<GTDefinitionId, GTPModuleDefinitionResolve>,
}
