use crate::prelude::internal::*;

impl PYTraverse for PYExtension {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_extension(self);
        self.reference.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut extension = PYExtension {
            reference: PYReference {
                identifier: PYIdentifier("Name".into()).into(),
                forward: false,
            },
        };
        extension.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Extension(extension.clone()),
                PYMockVisited::Reference(extension.reference.clone()),
                PYMockVisited::Identifier(extension.reference.identifier),
            ]
        );
    }
}
