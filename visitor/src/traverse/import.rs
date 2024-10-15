use genotype_parser::tree::import::GTImport;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImport {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import(self);
        self.path.traverse(visitor);
        self.reference.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::visitor::mock::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let path = GTPath::parse((0, 0).into(), "./path/to/module").unwrap();
        let reference = GTImportReference::Glob;
        let mut import = GTImport {
            span: (0, 0).into(),
            path: path.clone(),
            reference: reference.clone(),
        };
        import.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Import(import.clone()),
                GTMockVisited::Path(path.clone()),
                GTMockVisited::ImportReference(reference.clone()),
            ]
        );
    }
}
