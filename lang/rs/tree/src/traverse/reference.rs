use crate::prelude::internal::*;

impl RSTraverse for RSReference {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_reference(self);
        self.identifier.traverse(visitor);
    }
}
