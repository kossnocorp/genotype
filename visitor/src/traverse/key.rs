use genotype_parser::tree::key::GTKey;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTKey {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_key(&self);
    }
}
