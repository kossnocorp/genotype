use genotype_parser::tree::GTLiteral;

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
        let mut literal = GTLiteral::String("Hello, world!".into());
        literal.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::Literal(literal.clone()),]
        );
    }
}
