use crate::prelude::internal::*;

impl RSTraverse for RSAny {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_any(self);
    }
}
