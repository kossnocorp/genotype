use genotype_lang_py_tree::property::PYProperty;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYProperty {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_property(self);
        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }
        self.name.traverse(visitor);
        self.descriptor.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_base() {
        let mut visitor = PYMockVisitor::new();
        let mut property = PYProperty {
            doc: None,
            name: PYKey("key".into()),
            descriptor: PYPrimitive::String.into(),
            required: true,
        };
        property.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Property(property.clone()),
                PYMockVisited::Key(property.name.clone()),
                PYMockVisited::Descriptor(property.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = PYMockVisitor::new();
        let mut property = PYProperty {
            doc: Some(PYDoc("Hello, world!".into())),
            name: PYKey("key".into()),
            descriptor: PYPrimitive::String.into(),
            required: true,
        };
        property.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Property(property.clone()),
                PYMockVisited::Doc(property.doc.clone().unwrap()),
                PYMockVisited::Key(property.name.clone()),
                PYMockVisited::Descriptor(property.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }
}
