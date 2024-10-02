use std::collections::HashMap;

use genotype_parser::tree::{reference, GTIdentifier, GTReference};
use genotype_visitor::visitor::GTVisitor;

use crate::GTProjectPath;

pub struct GTProjectResolveVisitor {
    exports: HashMap<GTProjectPath, Vec<GTIdentifier>>,
}

impl GTProjectResolveVisitor {
    pub fn new(exports: HashMap<GTProjectPath, Vec<GTIdentifier>>) -> Self {
        Self { exports }
    }
}

impl GTVisitor for GTProjectResolveVisitor {
    fn visit_reference(&mut self, reference: &mut GTReference) {
        if let GTReference::Unresolved(identifier) = reference {
            *reference = GTReference::External(identifier.clone(), "[TODO]".into());
        }
    }
}
