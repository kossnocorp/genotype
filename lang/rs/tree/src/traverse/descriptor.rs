use crate::prelude::internal::*;

impl RSTraverse for RSDescriptor {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_descriptor(self);

        match self {
            RSDescriptor::Enum(r#enum) => r#enum.traverse(visitor),

            RSDescriptor::Vec(vec) => vec.traverse(visitor),

            RSDescriptor::Primitive(primitive) => primitive.traverse(visitor),

            RSDescriptor::Reference(reference) => reference.traverse(visitor),

            RSDescriptor::InlineUse(inline_use) => inline_use.traverse(visitor),

            RSDescriptor::Tuple(tuple) => tuple.traverse(visitor),

            RSDescriptor::Map(map) => map.traverse(visitor),

            RSDescriptor::Option(option) => option.traverse(visitor),

            RSDescriptor::Any(any) => any.traverse(visitor),
        }
    }
}
