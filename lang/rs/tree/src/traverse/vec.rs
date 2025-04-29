use crate::prelude::internal::*;

impl RSTraverse for RSVec {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_vec(self);
        self.descriptor.traverse(visitor);
    }
}
