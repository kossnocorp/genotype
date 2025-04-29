use crate::prelude::internal::*;

impl RSTraverse for RSTuple {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_tuple(self);

        for descriptor in &mut self.descriptors {
            descriptor.traverse(visitor);
        }
    }
}
