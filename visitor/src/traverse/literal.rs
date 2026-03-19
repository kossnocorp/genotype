use genotype_parser::tree::{GTLiteral, GTLiteralValue};

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTLiteral {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_literal(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let value = GTLiteralValue::String("Hello, world!".into());
        let mut literal = GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value,
        };
        literal.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::Literal(literal.clone()),]
        );
    }
}
