use crate::prelude::internal::*;

impl PYTraverse for PYImportReference {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_import_reference(self);
        match self {
            PYImportReference::Default(Some(identifier)) => {
                identifier.traverse(visitor);
            }

            PYImportReference::Named(names) => {
                for name in names {
                    name.traverse(visitor);
                }
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
    fn test_traverse_default_none() {
        let mut visitor = PYMockVisitor::new();
        let mut reference = PYImportReference::Default(None);
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::ImportReference(reference),]
        );
    }

    #[test]
    fn test_traverse_default_some() {
        let mut visitor = PYMockVisitor::new();
        let identifier = PYIdentifier("Name".into());
        let mut reference = PYImportReference::Default(Some(identifier.clone()));
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::ImportReference(reference),
                PYMockVisited::Identifier(identifier)
            ]
        );
    }

    #[test]
    fn test_traverse_glob() {
        let mut visitor = PYMockVisitor::new();
        let mut reference = PYImportReference::Glob;
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::ImportReference(reference),]
        );
    }

    #[test]
    fn test_traverse_named() {
        let mut visitor = PYMockVisitor::new();
        let identifier = PYIdentifier("Name".into());
        let name = PYImportName::Name(identifier.clone());
        let mut reference = PYImportReference::Named(vec![name.clone()]);
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::ImportReference(reference),
                PYMockVisited::ImportName(name),
                PYMockVisited::Identifier(identifier),
            ]
        );
    }
}
