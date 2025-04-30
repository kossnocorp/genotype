use crate::prelude::internal::*;

impl PYTraverse for PYImport {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_import(self);
        self.reference.traverse(visitor);
        self.dependency.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::vec;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("./path/to/module".into());
        let reference = PYImportReference::Glob;
        let dependency = PYDependencyIdent::Path(path.clone());
        let mut import = PYImport {
            reference: reference.clone(),
            dependency: dependency.clone(),
        };
        import.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Import(import.clone()),
                PYMockVisited::ImportReference(reference),
                PYMockVisited::Dependency(dependency),
                PYMockVisited::Path(path),
            ]
        );
    }

    #[test]
    fn test_traverse_dependency() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("./path/to/module".into());
        let reference = PYImportReference::Glob;
        let dependency = PYDependencyIdent::Path(path.clone());
        let mut import = PYImport {
            reference: reference.clone(),
            dependency: dependency.clone(),
        };
        import.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Import(import.clone()),
                PYMockVisited::ImportReference(reference.clone()),
                PYMockVisited::Dependency(dependency.clone()),
                PYMockVisited::Path(path.clone()),
            ]
        );
    }
}
