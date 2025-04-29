use crate::prelude::internal::*;

impl RSTraverse for RSMap {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_map(self);

        self.key.traverse(visitor);
        self.descriptor.traverse(visitor);
    }
}
