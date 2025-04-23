use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSDoc {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_doc(self);
    }
}
