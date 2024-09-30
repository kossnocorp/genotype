use genotype_parser::tree::module::GTModule;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTModule {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_module(self);

        for import in &self.imports {
            import.traverse(visitor);
        }

        for alias in &self.aliases {
            alias.traverse(visitor);
        }
    }
}
