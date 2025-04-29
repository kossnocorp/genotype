use crate::prelude::internal::*;

impl RSTraverse for RSDefinition {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_definition(self);

        match self {
            RSDefinition::Alias(alias) => alias.traverse(visitor),
            RSDefinition::Struct(r#struct) => r#struct.traverse(visitor),
            RSDefinition::Enum(r#enum) => r#enum.traverse(visitor),
        }
    }
}
