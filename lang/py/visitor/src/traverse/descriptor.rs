use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYDescriptor {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_descriptor(self);

        match self {
            PYDescriptor::List(list) => list.traverse(visitor),

            PYDescriptor::Literal(literal) => literal.traverse(visitor),

            PYDescriptor::Primitive(primitive) => primitive.traverse(visitor),

            PYDescriptor::Reference(reference) => reference.traverse(visitor),

            PYDescriptor::Tuple(tuple) => tuple.traverse(visitor),

            PYDescriptor::Union(union) => union.traverse(visitor),

            PYDescriptor::Dict(dict) => dict.traverse(visitor),

            PYDescriptor::Any(any) => any.traverse(visitor),
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
    fn test_traverse_list() {
        let mut visitor = PYMockVisitor::new();
        let list = PYList {
            descriptor: PYPrimitive::String.into(),
        };
        let mut descriptor = PYDescriptor::List(Box::new(list.clone()));
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::List(list.clone()),
                PYMockVisited::Descriptor(list.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }

    #[test]
    fn test_traverse_literal() {
        let mut visitor = PYMockVisitor::new();
        let literal = PYLiteral::String("Hello, world!".into());
        let mut descriptor = PYDescriptor::Literal(literal.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Literal(literal.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_primitive() {
        let mut visitor = PYMockVisitor::new();
        let primitive = PYPrimitive::String;
        let mut descriptor = PYDescriptor::Primitive(primitive.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }

    #[test]
    fn test_traverse_reference() {
        let mut visitor = PYMockVisitor::new();
        let identifier = PYIdentifier("Name".into());
        let reference = PYReference {
            identifier: identifier.clone(),
            forward: true,
        };
        let mut descriptor = PYDescriptor::Reference(reference.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Reference(reference.clone()),
                PYMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_tuple() {
        let mut visitor = PYMockVisitor::new();
        let primitive = PYDescriptor::Primitive(PYPrimitive::String);
        let tuple = PYTuple {
            descriptors: vec![primitive.clone()],
        };
        let mut descriptor = PYDescriptor::Tuple(tuple.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Tuple(tuple.clone()),
                PYMockVisited::Descriptor(primitive),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }

    #[test]
    fn test_traverse_union() {
        let mut visitor = PYMockVisitor::new();
        let primitive = PYDescriptor::Primitive(PYPrimitive::String);
        let union = PYUnion {
            descriptors: vec![primitive.clone()],
            discriminator: None,
        };
        let mut descriptor = PYDescriptor::Union(union.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Union(union.clone()),
                PYMockVisited::Descriptor(primitive),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }

    #[test]
    fn test_traverse_dict() {
        let mut visitor = PYMockVisitor::new();
        let key = PYDictKey::String;
        let primitive = PYPrimitive::String;
        let primitive_descriptor = PYDescriptor::Primitive(primitive.clone());
        let dict = PYDict {
            key: key.clone(),
            descriptor: primitive_descriptor.clone(),
        };
        let mut descriptor = PYDescriptor::Dict(Box::new(dict.clone()));
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Dict(dict.clone()),
                PYMockVisited::DictKey(key.clone()),
                PYMockVisited::Descriptor(primitive_descriptor.clone()),
                PYMockVisited::Primitive(primitive.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_any() {
        let mut visitor = PYMockVisitor::new();
        let mut any = PYAny;
        any.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::Any(any.clone()),]);
    }
}
