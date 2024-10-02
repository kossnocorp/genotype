use genotype_parser::tree::union::GTUnion;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTUnion {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_union(&self);
    }
}
