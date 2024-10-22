use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAttributeAssignment {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_attribute_assignment(self);

        self.value.traverse(visitor);
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
        let literal = GTLiteral::String((0, 0).into(), "answer".into());
        let value = GTAttributeValue::Literal(literal.clone());
        let mut assignment = GTAttributeAssignment::new((0, 0).into(), value.clone());
        assignment.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::AttributeAssignment(assignment.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone())
            ]
        );
    }
}
