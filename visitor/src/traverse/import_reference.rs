use genotype_parser::tree::import_reference::GTImportReference;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImportReference {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import_reference(self);
        match self {
            GTImportReference::Glob(_) => {}
            GTImportReference::Names(_, names) => {
                for name in names {
                    name.traverse(visitor);
                }
            }
            GTImportReference::Name(_, name) => {
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
        let mut reference = GTImportReference::Glob((0, 0).into());
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::ImportReference(reference.clone()),]
        );
    }

    #[test]
    fn test_traverse_names() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let import_name = GTImportName::Name((0, 0).into(), identifier.clone());
        let mut reference = GTImportReference::Names((0, 0).into(), vec![import_name.clone()]);
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
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let mut reference = GTImportReference::Name((0, 0).into(), identifier.clone());
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
