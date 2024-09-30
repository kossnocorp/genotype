use genotype_parser::tree::name::GTName;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTName {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_name(&self);
    }
}
