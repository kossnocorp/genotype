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

            GTDescriptor::Object(object) => object.traverse(visitor),

            GTDescriptor::Primitive(primitive) => primitive.traverse(visitor),

            GTDescriptor::Reference(reference) => reference.traverse(visitor),

            GTDescriptor::Tuple(tuple) => tuple.traverse(visitor),

            GTDescriptor::Union(union) => union.traverse(visitor),

            GTDescriptor::Record(record) => record.traverse(visitor),

            GTDescriptor::Any(any) => any.traverse(visitor),
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
            id: GTAliasId("module".into(), "Name".into()),
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
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
            span: (0, 0).into(),
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
            span: (0, 0).into(),
            path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
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
    fn test_traverse_object() {
        let mut visitor = GTMockVisitor::new();
        let property = GTProperty {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTKey((0, 0).into(), "key".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
            required: true,
        };
        let name_identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let name = GTObjectName::Named(name_identifier.clone());
        let object = GTObject {
            span: (0, 0).into(),
            name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Name".into())),
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
                GTMockVisited::ObjectName(name.clone()),
                GTMockVisited::Identifier(name_identifier.clone()),
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
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let reference = GTReference((0, 0).into(), identifier.clone());
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
            span: (0, 0).into(),
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
            span: (0, 0).into(),
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

    #[test]
    fn test_traverse_record() {
        let mut visitor = GTMockVisitor::new();
        let key = GTRecordKey::String((0, 0).into());
        let primitive = GTPrimitive::String((0, 0).into());
        let primitive_descriptor = GTDescriptor::Primitive(primitive.clone());
        let record = GTRecord {
            span: (0, 0).into(),
            key: key.clone(),
            descriptor: primitive_descriptor.clone(),
        };
        let mut descriptor = GTDescriptor::Record(Box::new(record.clone()));
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Record(record.clone()),
                GTMockVisited::RecordKey(key.clone()),
                GTMockVisited::Descriptor(primitive_descriptor.clone()),
                GTMockVisited::Primitive(primitive.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_any() {
        let mut visitor = GTMockVisitor::new();
        let mut any = GTAny((0, 0).into());
        any.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![GTMockVisited::Any(any.clone()),]);
    }
}
