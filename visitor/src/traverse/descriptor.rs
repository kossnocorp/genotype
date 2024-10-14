use genotype_parser::tree::descriptor::GTDescriptor;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTDescriptor {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_descriptor(self);

        match self {
            GTDescriptor::Alias(alias) => alias.traverse(visitor),

            GTDescriptor::Array(array) => array.traverse(visitor),

            GTDescriptor::InlineImport(import) => import.traverse(visitor),

            GTDescriptor::Literal(literal) => literal.traverse(visitor),

            GTDescriptor::Nullable(descriptor) => descriptor.traverse(visitor),

            GTDescriptor::Object(object) => object.traverse(visitor),

            GTDescriptor::Primitive(primitive) => primitive.traverse(visitor),

            GTDescriptor::Reference(reference) => reference.traverse(visitor),

            GTDescriptor::Tuple(tuple) => tuple.traverse(visitor),

            GTDescriptor::Union(union) => union.traverse(visitor),
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
    fn test_traverse_alias() {
        let mut visitor = GTMockVisitor::new();
        let alias = GTAlias {
            doc: None,
            name: "Name".into(),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        let mut descriptor = GTDescriptor::Alias(Box::new(alias.clone()));
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Alias(alias.clone()),
                GTMockVisited::Descriptor(alias.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_array() {
        let mut visitor = GTMockVisitor::new();
        let array = GTArray {
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        let mut descriptor = GTDescriptor::Array(Box::new(array.clone()));
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Array(array.clone()),
                GTMockVisited::Descriptor(array.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_inline_import() {
        let mut visitor = GTMockVisitor::new();
        let import = GTInlineImport {
            path: "./path/to/module".into(),
            name: "Name".into(),
        };
        let mut descriptor = GTDescriptor::InlineImport(import.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::InlineImport(import.clone()),
                GTMockVisited::Identifier(import.name.clone()),
                GTMockVisited::Path(import.path.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_nullable() {
        let mut visitor = GTMockVisitor::new();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String((0, 0).into()));
        let mut descriptor = GTDescriptor::Nullable(Box::new(primitive.clone()));
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Descriptor(primitive.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_object() {
        let mut visitor = GTMockVisitor::new();
        let property = GTProperty {
            doc: None,
            name: GTKey("key".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
            required: true,
        };
        let object = GTObject {
            extensions: vec![],
            properties: vec![property.clone()],
        };
        let mut descriptor = GTDescriptor::Object(object.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Object(object.clone()),
                GTMockVisited::Property(property.clone()),
                GTMockVisited::Key(property.name.clone()),
                GTMockVisited::Descriptor(property.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_primitive() {
        let mut visitor = GTMockVisitor::new();
        let primitive = GTPrimitive::String((0, 0).into());
        let mut descriptor = GTDescriptor::Primitive(primitive.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_reference() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier("Name".into());
        let reference = GTReference(identifier.clone());
        let mut descriptor = GTDescriptor::Reference(reference.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Reference(reference.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_tuple() {
        let mut visitor = GTMockVisitor::new();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String((0, 0).into()));
        let tuple = GTTuple {
            descriptors: vec![primitive.clone()],
        };
        let mut descriptor = GTDescriptor::Tuple(tuple.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Tuple(tuple.clone()),
                GTMockVisited::Descriptor(primitive),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_union() {
        let mut visitor = GTMockVisitor::new();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String((0, 0).into()));
        let union = GTUnion {
            descriptors: vec![primitive.clone()],
        };
        let mut descriptor = GTDescriptor::Union(union.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Union(union.clone()),
                GTMockVisited::Descriptor(primitive),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
