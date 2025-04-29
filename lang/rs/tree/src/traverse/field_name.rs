use crate::prelude::internal::*;

impl RSTraverse for RSFieldName {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_field_name(self);
    }
}
