use crate::prelude::internal::*;

impl PYTraverse for PYLiteral {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_literal(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut literal = PYLiteral::String("Hello, world!".into());
        literal.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Literal(literal.clone()),]
        );
    }
}
