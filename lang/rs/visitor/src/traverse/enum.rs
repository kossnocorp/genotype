use genotype_lang_rs_tree::RSEnum;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSEnum {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for attribute in &mut self.attributes {
            attribute.traverse(visitor);
        }

        self.name.traverse(visitor);

        for variant in &mut self.variants {
            variant.traverse(visitor);
        }
    }
}
