use genotype_parser::tree::array::GTArray;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTArray {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_array(self);
        self.descriptor.traverse(visitor);
    }
}
