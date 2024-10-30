use genotype_lang_py_tree::import::PYImport;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYImport {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_import(self);
        self.path.traverse(visitor);
        self.reference.traverse(visitor);
        self.dependency.traverse(visitor);
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
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("./path/to/module".into());
        let reference = PYImportReference::Glob;
        let mut import = PYImport {
            path: path.clone(),
            reference: reference.clone(),
            dependency: PYDependency::Local(path.clone()),
        };
        import.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Import(import.clone()),
                PYMockVisited::Path(path.clone()),
                PYMockVisited::ImportReference(reference.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_dependency() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("./path/to/module".into());
        let reference = PYImportReference::Glob;
        let mut import = PYImport {
            path: path.clone(),
            reference: reference.clone(),
            dependency: PYDependency::Local(path.clone()),
        };
        import.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Import(import.clone()),
                PYMockVisited::Path(path.clone()),
                PYMockVisited::ImportReference(reference.clone()),
            ]
        );
    }
}
