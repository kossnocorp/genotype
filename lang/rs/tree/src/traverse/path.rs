use crate::prelude::internal::*;

impl RSTraverse for RSPath {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_path(self);
    }
}
