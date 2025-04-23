use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSDependencyIdent {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_dependency(self);

        match self {
            RSDependencyIdent::Local(path) => {
                path.traverse(visitor);
            }

            _ => {}
        }
    }
}
