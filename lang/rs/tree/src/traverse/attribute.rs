use crate::prelude::internal::*;

impl RSTraverse for RSAttribute {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_attribute(self);
    }
}
