use genotype_parser::tree::alias::GTAlias;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAlias {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_alias(self);
        self.descriptor.traverse(visitor);
    }
}
