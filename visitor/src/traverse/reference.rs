use genotype_parser::tree::reference::GTReference;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTReference {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_reference(&self);
    }
}
