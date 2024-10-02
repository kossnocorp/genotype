use genotype_parser::tree::descriptor::GTDescriptor;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTDescriptor {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_descriptor(&self);

        match self {
            GTDescriptor::Primitive(primitive) => primitive.traverse(visitor),

            GTDescriptor::Reference(reference) => reference.traverse(visitor),

            GTDescriptor::Nullable(descriptor) => descriptor.traverse(visitor),

            GTDescriptor::Object(object) => object.traverse(visitor),

            GTDescriptor::Array(array) => array.traverse(visitor),

            GTDescriptor::Tuple(tuple) => tuple.traverse(visitor),

            GTDescriptor::Alias(alias) => alias.traverse(visitor),

            GTDescriptor::InlineImport(import) => import.traverse(visitor),
        }
    }
}
