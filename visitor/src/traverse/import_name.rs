use genotype_parser::tree::import_name::GTImportName;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImportName {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import_name(&self);
    }
}
