use genotype_lang_rs_tree::RSDependency;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSDependency {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_dependency(self);

        match self {
            RSDependency::Local(path) => {
                path.traverse(visitor);
            }

            _ => {}
        }
    }
}
