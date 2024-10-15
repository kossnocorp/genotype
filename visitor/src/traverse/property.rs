use genotype_parser::tree::property::GTProperty;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTProperty {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
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
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_base() {
        let mut visitor = GTMockVisitor::new();
        let mut property = GTProperty {
            span: (0, 0).into(),
            doc: None,
            name: GTKey((0, 0).into(), "key".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
            required: true,
        };
        property.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Property(property.clone()),
                GTMockVisited::Key(property.name.clone()),
                GTMockVisited::Descriptor(property.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = GTMockVisitor::new();
        let mut property = GTProperty {
            span: (0, 0).into(),
            doc: Some(GTDoc("Hello, world!".into())),
            name: GTKey((0, 0).into(), "key".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
            required: true,
        };
        property.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Property(property.clone()),
                GTMockVisited::Doc(property.doc.clone().unwrap()),
                GTMockVisited::Key(property.name.clone()),
                GTMockVisited::Descriptor(property.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
