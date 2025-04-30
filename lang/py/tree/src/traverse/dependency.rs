use crate::prelude::internal::*;

impl PYTraverse for PYDependencyIdent {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_dependency(self);

        match self {
            PYDependencyIdent::Path(path) => {
                path.traverse(visitor);
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::vec;

    #[test]
    fn test_traverse_local() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("path".into());
        let mut dependency = PYDependencyIdent::Path(path.clone());
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
        let mut dependency = PYDependencyIdent::Runtime;
        dependency.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Dependency(dependency.clone()),]
        );
    }
}
