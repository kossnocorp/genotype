use crate::prelude::internal::*;

impl RSTraverse for RSDoc {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_doc(self);
    }
}
