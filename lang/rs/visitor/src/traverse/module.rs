use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSModule {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_module(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for import in &mut self.imports {
            import.traverse(visitor);
        }

        for definitions in &mut self.definitions {
            definitions.traverse(visitor);
        }
    }
}
