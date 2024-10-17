use genotype_parser::tree::object::GTObject;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTObject {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_object(self);
        for extension in &mut self.extensions {
            extension.traverse(visitor);
        }
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
        let extension = GTExtension {
            span: (0, 0).into(),
            reference: GTIdentifier::new((0, 0).into(), "Name".into()).into(),
        };
        let property = GTProperty {
            span: (0, 0).into(),
            doc: None,
            name: GTKey((0, 0).into(), "key".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
            required: true,
        };
        let mut object = GTObject {
            span: (0, 0).into(),
            name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Name".into())),
            extensions: vec![extension.clone()],
            properties: vec![property.clone()],
        };
        object.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Object(object.clone()),
                GTMockVisited::Extension(extension.clone()),
                GTMockVisited::Reference(extension.reference.clone()),
                GTMockVisited::Identifier(extension.reference.1.clone()),
                GTMockVisited::Property(property.clone()),
                GTMockVisited::Key(property.name.clone()),
                GTMockVisited::Descriptor(property.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
