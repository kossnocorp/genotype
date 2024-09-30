use genotype_parser::tree::object::GTObject;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTObject {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_object(self);
        for property in &self.properties {
            property.traverse(visitor);
        }
    }
}
