use genotype_lang_rs_tree::RSOption;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSOption {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_option(self);
        self.descriptor.traverse(visitor);
    }
}
