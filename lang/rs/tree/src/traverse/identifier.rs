use crate::prelude::internal::*;

impl RSTraverse for RSIdentifier {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_identifier(self);
    }
}
