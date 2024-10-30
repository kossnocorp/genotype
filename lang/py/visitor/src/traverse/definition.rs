use genotype_lang_py_tree::PYDefinition;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYDefinition {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_definition(self);

        match self {
            PYDefinition::Alias(alias) => alias.traverse(visitor),
            PYDefinition::Class(class) => class.traverse(visitor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_alias() {
        let mut visitor = PYMockVisitor::new();
        let reference = PYIdentifier("Reference".into());
        let alias = PYAlias {
            doc: None,
            name: PYIdentifier("Name".into()),
            descriptor: PYPrimitive::String.into(),
            references: vec![reference.clone()],
        };
        let mut definition = PYDefinition::Alias(alias.clone());
        definition.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Definition(definition.clone()),
                PYMockVisited::Alias(alias.clone()),
                PYMockVisited::Descriptor(alias.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
                PYMockVisited::Identifier(reference)
            ]
        );
    }

    #[test]
    fn test_traverse_class() {
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
        let class = PYClass {
            doc: None,
            name: name_identifier.clone(),
            extensions: vec![extension.clone()],
            properties: vec![property.clone()],
            references: vec![reference.clone()],
        };
        let mut definition = PYDefinition::Class(class.clone());
        definition.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Definition(definition.clone()),
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
