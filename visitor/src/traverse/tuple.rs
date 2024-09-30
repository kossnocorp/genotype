use genotype_parser::tree::tuple::GTTuple;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTTuple {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_tuple(self);
        for descriptor in &self.descriptors {
            descriptor.traverse(visitor);
        }
    }
}
