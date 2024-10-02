use genotype_parser::tree::identifier::GTIdentifier;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTIdentifier {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_identifier(&self);
    }
}
