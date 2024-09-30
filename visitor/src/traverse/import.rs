use genotype_parser::tree::import::GTImport;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImport {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import(self);
    }
}
