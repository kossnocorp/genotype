use genotype_parser::tree::primitive::GTPrimitive;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTPrimitive {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_primitive(self);
    }
}
