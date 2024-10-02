use genotype_parser::tree::import_reference::GTImportReference;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImportReference {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import_reference(&self);
        match self {
            GTImportReference::Glob => {}
            GTImportReference::Names(names) => {
                for name in names {
                    name.traverse(visitor);
                }
            }
            GTImportReference::Name(name) => {
                name.traverse(visitor);
            }
        }
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
    fn test_traverse_glob() {
        let mut visitor = GTMockVisitor::new();
        let reference = GTImportReference::Glob;
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::ImportReference(reference.clone()),]
        );
    }

    #[test]
    fn test_traverse_names() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier("Name".into());
        let import_name = GTImportName::Name(identifier.clone());
        let reference = GTImportReference::Names(vec![import_name.clone()]);
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ImportReference(reference.clone()),
                GTMockVisited::ImportName(import_name.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_name() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier("Name".into());
        let reference = GTImportReference::Name(identifier.clone());
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ImportReference(reference.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }
}
