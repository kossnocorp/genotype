use genotype_parser::GTBranded;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTBranded {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_branded(self);
        self.name.traverse(visitor);
        self.primitive.traverse(visitor);
    }
}
