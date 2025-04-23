use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYLiteral {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
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
        let mut visitor = PYMockVisitor::new();
        let mut literal = PYLiteral::String("Hello, world!".into());
        literal.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Literal(literal.clone()),]
        );
    }
}
