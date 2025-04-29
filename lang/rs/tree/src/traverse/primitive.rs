use crate::prelude::internal::*;

impl RSTraverse for RSPrimitive {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_primitive(self);
    }
}
