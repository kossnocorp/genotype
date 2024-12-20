use genotype_lang_rs_tree::alias::RSAlias;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSAlias {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_alias(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        self.name.traverse(visitor);

        self.descriptor.traverse(visitor);
    }
}
