use genotype_lang_rs_tree::path::RSPath;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSPath {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_path(self);
    }
}
