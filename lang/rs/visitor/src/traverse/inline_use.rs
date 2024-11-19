use genotype_lang_rs_tree::RSInlineUse;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSInlineUse {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_inline_use(self);
        self.path.traverse(visitor);
        self.name.traverse(visitor);
    }
}
