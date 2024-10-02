use genotype_parser::tree::reference::GTReference;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTReference {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_reference(self);
        match self {
            GTReference::Unresolved(identifier) => {
                identifier.traverse(visitor);
            }

            GTReference::Local(identifier) => {
                identifier.traverse(visitor);
            }

            GTReference::External(identifier, path) => {
                identifier.traverse(visitor);
                path.traverse(visitor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_unresolved() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier("Name".into());
        let mut reference = GTReference::Unresolved(identifier.clone());
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Reference(reference.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_local() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier("Name".into());
        let mut reference = GTReference::Local(identifier.clone());
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Reference(reference.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_external() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier("Name".into());
        let path = GTPath("./path/to/module".into());
        let mut reference = GTReference::External(identifier.clone(), path.clone());
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Reference(reference.clone()),
                GTMockVisited::Identifier(identifier.clone()),
                GTMockVisited::Path(path.clone()),
            ]
        );
    }
}
