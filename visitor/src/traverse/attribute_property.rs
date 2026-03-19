use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAttributeProperty {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_attribute_property(self);

        self.name.traverse(visitor);
        self.value.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_test::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let key = GTAttributeKey::new((0, 0).into(), "question".into());
        let literal = GtFactory::literal_string("answer");
        let value = GTAttributeValue::Literal(literal.clone());
        let mut property = GTAttributeProperty::new((0, 0).into(), key.clone(), value.clone());
        property.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::AttributeProperty(property.clone()),
                GTMockVisited::AttributeKey(key.clone()),
                GTMockVisited::AttributeValue(value.clone()),
                GTMockVisited::Literal(literal.clone())
            ]
        );
    }
}
