use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAttributeDescriptor {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_attribute_descriptor(self);

        match self {
            GTAttributeDescriptor::Assignment(assignment) => {
                assignment.traverse(visitor);
            }

            GTAttributeDescriptor::Arguments(arguments) => {
                for argument in arguments {
                    argument.traverse(visitor);
                }
            }

            GTAttributeDescriptor::Properties(properties) => {
                for property in properties {
                    property.traverse(visitor);
                }
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
    fn test_traverse_assignment() {
        let mut visitor = GTMockVisitor::new();
        let literal = GTLiteral::String((0, 0).into(), "answer".into());
        let value = GTAttributeValue::Literal(literal.clone());
        let assignment = GTAttributeAssignment::new((0, 0).into(), value.clone());
        let mut descriptor = GTAttributeDescriptor::Assignment(assignment.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
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
        let mut descriptor = GTAttributeDescriptor::Arguments(vec![value.clone(), value.clone()]);
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::AttributeDescriptor(descriptor.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone())
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
        let mut descriptor =
            GTAttributeDescriptor::Properties(vec![property.clone(), property.clone()]);
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::AttributeDescriptor(descriptor.clone()),
                GTMockVisited::AttributeProperty(property.clone()),
                GTMockVisited::AttributeKey(key.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone()),
                GTMockVisited::AttributeProperty(property.clone()),
                GTMockVisited::AttributeKey(key.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone())
            ]
        );
    }
}
