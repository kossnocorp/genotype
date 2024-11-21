use genotype_lang_rs_tree::RSDefinition;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSDefinition {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_definition(self);

        match self {
            RSDefinition::Alias(alias) => alias.traverse(visitor),
            RSDefinition::Struct(r#struct) => r#struct.traverse(visitor),
            RSDefinition::Enum(r#enum) => r#enum.traverse(visitor),
            RSDefinition::Newtype(newtype) => newtype.traverse(visitor),
        }
    }
}
