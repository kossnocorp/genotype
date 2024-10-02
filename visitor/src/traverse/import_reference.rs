use genotype_parser::tree::import_reference::GTImportReference;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImportReference {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import_reference(&self);
    }
}
