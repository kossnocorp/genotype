use genotype_parser::tree::inline_import::GTInlineImport;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTInlineImport {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_inline_import(self);
    }
}
