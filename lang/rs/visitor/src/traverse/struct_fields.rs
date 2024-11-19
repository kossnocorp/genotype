use genotype_lang_rs_tree::RSStructFields;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSStructFields {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_struct_fields(self);

        match self {
            RSStructFields::Resolved(fields) => {
                for field in fields {
                    field.traverse(visitor);
                }
            }

            RSStructFields::Unresolved(_, reference_pairs, fields) => {
                for (_, reference) in reference_pairs {
                    reference.traverse(visitor);
                }

                for field in fields {
                    field.traverse(visitor);
                }
            }
        }
    }
}
