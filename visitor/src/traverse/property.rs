use genotype_parser::tree::property::GTProperty;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTProperty {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_property(self);
        self.descriptor.traverse(visitor);
    }
}
