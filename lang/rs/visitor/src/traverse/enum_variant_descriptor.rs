use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSEnumVariantDescriptor {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_enum_variant_descriptor(self);

        match self {
            RSEnumVariantDescriptor::Descriptor(descriptor) => descriptor.traverse(visitor),
        }
    }
}
