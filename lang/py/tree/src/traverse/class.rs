use crate::prelude::internal::*;

impl PYTraverse for PYClass {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_class(self);

        self.name.traverse(visitor);

        for extension in &mut self.extensions {
            extension.traverse(visitor);
        }

        for property in &mut self.properties {
            property.traverse(visitor);
        }

        for identifier in &mut self.references {
            identifier.traverse(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let extension = PYExtension {
            reference: PYReference {
                identifier: PYIdentifier("Base".into()),
                forward: true,
            },
        };
        let property = PYProperty {
            doc: None,
            name: PYKey("key".into()),
            descriptor: PYPrimitive::String.into(),
            required: true,
        };
        let name_identifier = PYIdentifier("Name".into());
        let reference = PYIdentifier("Reference".into());
        let mut class = PYClass {
            doc: None,
            name: name_identifier.clone(),
            extensions: vec![extension.clone()],
            properties: vec![property.clone()],
            references: vec![reference.clone()],
        };
        class.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Class(class.clone()),
                PYMockVisited::Identifier(name_identifier.clone()),
                PYMockVisited::Extension(extension.clone()),
                PYMockVisited::Reference(extension.reference.clone()),
                PYMockVisited::Identifier(extension.reference.identifier.clone()),
                PYMockVisited::Property(property.clone()),
                PYMockVisited::Key(property.name.clone()),
                PYMockVisited::Descriptor(property.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
                PYMockVisited::Identifier(reference.clone())
            ]
        );
    }
}
