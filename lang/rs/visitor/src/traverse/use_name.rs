use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSUseName {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_use_name(self);

        match self {
            RSUseName::Name(name) => name.traverse(visitor),

            RSUseName::Alias(name, alias) => {
                name.traverse(visitor);
                alias.traverse(visitor);
            }
        }
    }
}
