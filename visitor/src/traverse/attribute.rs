use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAttribute {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_attribute(self);

        self.name.traverse(visitor);

        if let Some(descriptor) = self.descriptor.as_mut() {
            descriptor.traverse(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_simple() {
        let mut visitor = GTMockVisitor::new();
        let mut attribute = GTAttribute::new(
            (0, 0).into(),
            GTAttributeName::new((0, 0).into(), "answer".into()).into(),
            None,
        );
        attribute.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Attribute(attribute.clone()),
                GTMockVisited::AttributeName(attribute.name.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_assignment() {
        let mut visitor = GTMockVisitor::new();
        let literal = GTLiteral::String((0, 0).into(), "answer".into());
        let value = GTAttributeValue::Literal(literal.clone());
        let assignment = GTAttributeAssignment::new((0, 0).into(), value.clone());
        let descriptor = GTAttributeDescriptor::Assigment(assignment.clone());
        let mut attribute = GTAttribute::new(
            (0, 0).into(),
            GTAttributeName::new((0, 0).into(), "answer".into()).into(),
            Some(descriptor.clone()),
        );
        attribute.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Attribute(attribute.clone()),
                GTMockVisited::AttributeName(attribute.name.clone()),
                GTMockVisited::AttributeDescriptor(descriptor.clone()),
                GTMockVisited::AttributeAssignment(assignment.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone())
            ]
        );
    }

    #[test]
    fn test_traverse_arguments() {
        let mut visitor = GTMockVisitor::new();
        let literal = GTLiteral::String((0, 0).into(), "answer".into());
        let value = GTAttributeValue::Literal(literal.clone());
        let descriptor = GTAttributeDescriptor::Arguments(vec![value.clone()]);
        let mut attribute = GTAttribute::new(
            (0, 0).into(),
            GTAttributeName::new((0, 0).into(), "answer".into()).into(),
            Some(descriptor.clone()),
        );
        attribute.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Attribute(attribute.clone()),
                GTMockVisited::AttributeName(attribute.name.clone()),
                GTMockVisited::AttributeDescriptor(descriptor.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_properties() {
        let mut visitor = GTMockVisitor::new();
        let key = GTAttributeKey::new((0, 0).into(), "question".into());
        let literal = GTLiteral::String((0, 0).into(), "answer".into());
        let value = GTAttributeValue::Literal(literal.clone());
        let property = GTAttributeProperty::new((0, 0).into(), key.clone(), value.clone());
        let descriptor = GTAttributeDescriptor::Properties(vec![property.clone()]);
        let mut attribute = GTAttribute::new(
            (0, 0).into(),
            GTAttributeName::new((0, 0).into(), "answer".into()).into(),
            Some(descriptor.clone()),
        );
        attribute.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Attribute(attribute.clone()),
                GTMockVisited::AttributeName(attribute.name.clone()),
                GTMockVisited::AttributeDescriptor(descriptor.clone()),
                GTMockVisited::AttributeProperty(property.clone()),
                GTMockVisited::AttributeKey(key.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone()),
            ]
        );
    }
}
