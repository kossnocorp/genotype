use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSField {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for attribute in &mut self.attributes {
            attribute.traverse(visitor);
        }

        self.name.traverse(visitor);
        self.descriptor.traverse(visitor);
    }
}
