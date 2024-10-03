use genotype_parser::tree::object::GTObject;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTObject {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_object(self);
        for property in &mut self.properties {
            property.traverse(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let property = GTProperty {
            doc: None,
            name: GTKey("key".into()),
            descriptor: GTPrimitive::String.into(),
            required: true,
        };
        let mut object = GTObject {
            properties: vec![property.clone()],
        };
        object.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Object(object.clone()),
                GTMockVisited::Property(property.clone()),
                GTMockVisited::Key(property.name.clone()),
                GTMockVisited::Descriptor(property.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String),
            ]
        );
    }
}
