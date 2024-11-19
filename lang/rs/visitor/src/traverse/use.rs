use genotype_lang_rs_tree::RSUse;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSUse {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_use(self);
        self.reference.traverse(visitor);
        self.dependency.traverse(visitor);
    }
}
