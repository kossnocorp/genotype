use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSVec {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_vec(self);
        self.descriptor.traverse(visitor);
    }
}
