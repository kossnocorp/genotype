use crate::prelude::internal::*;

impl RSTraverse for RSUseReference {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_use_reference(self);

        match self {
            RSUseReference::Named(names) => {
                for name in names {
                    name.traverse(visitor);
                }
            }
            _ => {}
        }
    }
}
