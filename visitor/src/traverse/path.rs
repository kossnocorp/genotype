use genotype_parser::tree::path::GTPath;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTPath {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_path(&self);
    }
}
