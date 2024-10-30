use genotype_lang_py_tree::PYDependency;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYDependency {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_dependency(self);

        match self {
            PYDependency::Local(path) => {
                path.traverse(visitor);
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_local() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("path".into());
        let mut dependency = PYDependency::Local(path.clone());
        dependency.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Dependency(dependency.clone()),
                PYMockVisited::Path(path),
            ]
        );
    }

    #[test]
    fn test_traverse_external() {
        let mut visitor = PYMockVisitor::new();
        let mut dependency = PYDependency::Runtime;
        dependency.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Dependency(dependency.clone()),]
        );
    }
}
