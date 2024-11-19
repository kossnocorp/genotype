use genotype_lang_rs_tree::RSFieldName;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSFieldName {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_field_name(self);
    }
}
