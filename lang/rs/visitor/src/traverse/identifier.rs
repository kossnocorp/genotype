use genotype_lang_rs_tree::identifier::RSIdentifier;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSIdentifier {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_identifier(self);
    }
}
