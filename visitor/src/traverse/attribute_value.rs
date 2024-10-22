use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAttributeValue {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_attribute_value(self);

        match self {
            GTAttributeValue::Literal(literal) => {
                literal.traverse(visitor);
            }

            GTAttributeValue::Identifier(identifier) => {
                identifier.traverse(visitor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_literal() {
        let mut visitor = GTMockVisitor::new();
        let literal = GTLiteral::String((0, 0).into(), "answer".into());
        let mut value = GTAttributeValue::Literal(literal.clone());
        value.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone())
            ]
        );
    }

    #[test]
    fn test_traverse_identifier() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "answer".into());
        let mut value = GTAttributeValue::Identifier(identifier.clone());
        value.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Identifier(identifier.clone())
            ]
        );
    }
}
