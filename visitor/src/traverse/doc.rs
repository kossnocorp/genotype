use genotype_parser::tree::doc::GTDoc;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTDoc {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_doc(&self);
    }
}
