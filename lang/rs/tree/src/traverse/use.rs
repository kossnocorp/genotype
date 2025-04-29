use crate::prelude::internal::*;

impl RSTraverse for RSUse {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_use(self);
        self.reference.traverse(visitor);
        self.dependency.traverse(visitor);
    }
}
