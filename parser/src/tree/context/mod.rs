use std::collections::HashSet;

use super::{GTIdentifier, GTKey, GTModuleId, GTResolve};

mod ids;
mod naming;

#[derive(Debug, PartialEq, Clone)]
pub struct GTContext {
    /// Current module id.
    pub module_id: GTModuleId,
    pub resolve: GTResolve,
    pub parents: Vec<GTContextParent>,
    /// A set of taken definition names. It allows to generate unique syntetic
    /// names.
    pub taken_names: HashSet<String>,
}

/// The parent context enum that defines the kind of a parent an object has.
/// It allows building object names from the parents.
#[derive(Debug, PartialEq, Clone)]
pub enum GTContextParent {
    /// An explicitely named alias parent.
    Alias(GTIdentifier),
    /// An anonymous parent, i.e. an union or a nested object.
    Anonymous,
    /// A property parent.
    Property(GTKey),
}

impl GTContext {
    pub fn new(module_id: GTModuleId) -> Self {
        GTContext {
            module_id,
            resolve: GTResolve::new(),
            parents: vec![],
            taken_names: HashSet::new(),
        }
    }
}
