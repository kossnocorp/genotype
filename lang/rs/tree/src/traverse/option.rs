use crate::prelude::internal::*;

impl RSTraverse for RSOption {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_option(self);
        self.descriptor.traverse(visitor);
    }
}
