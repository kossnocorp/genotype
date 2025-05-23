use crate::prelude::internal::*;

impl RSTraverse for RSStructFields {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_struct_fields(self);

        match self {
            RSStructFields::Newtype(descriptors) => {
                for descriptor in descriptors {
                    descriptor.traverse(visitor);
                }
            }

            RSStructFields::Resolved(fields) => {
                for field in fields {
                    field.traverse(visitor);
                }
            }

            RSStructFields::Unresolved(_, reference_pairs, fields) => {
                for reference in reference_pairs {
                    reference.traverse(visitor);
                }

                for field in fields {
                    field.traverse(visitor);
                }
            }

            RSStructFields::Unit => {}
        }
    }
}
