use genotype_lang_rs_tree::RSAttribute;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSAttribute {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_attribute(self);
    }
}
