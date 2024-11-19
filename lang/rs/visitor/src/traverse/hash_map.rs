use genotype_lang_rs_tree::RSHashMap;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSHashMap {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_hash_map(self);

        self.key.traverse(visitor);
        self.descriptor.traverse(visitor);
    }
}
