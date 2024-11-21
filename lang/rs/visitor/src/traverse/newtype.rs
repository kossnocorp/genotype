use genotype_lang_rs_tree::RSNewtype;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSNewtype {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_newtype(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for attribute in &mut self.attributes {
            attribute.traverse(visitor);
        }

        self.name.traverse(visitor);

        for descriptor in &mut self.descriptors {
            descriptor.traverse(visitor);
        }
    }
}
