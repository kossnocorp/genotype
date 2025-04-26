use crate::prelude::internal::*;

impl PYTraverse for PYIdentifier {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_identifier(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut identifier = PYIdentifier("Name".into());
        identifier.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Identifier(identifier.clone()),]
        );
    }
}
