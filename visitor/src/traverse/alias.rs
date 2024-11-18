use genotype_parser::tree::alias::GTAlias;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAlias {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_alias(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for attribute in &mut self.attributes {
            attribute.traverse(visitor);
        }

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
        let mut alias = GTAlias {
            id: GTDefinitionId("module".into(), "Name".into()),
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        alias.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Alias(alias.clone()),
                GTMockVisited::Descriptor(alias.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = GTMockVisitor::new();
        let mut alias = GTAlias {
            id: GTDefinitionId("module".into(), "Name".into()),
            span: (0, 0).into(),
            doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        alias.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Alias(alias.clone()),
                GTMockVisited::Doc(alias.doc.clone().unwrap()),
                GTMockVisited::Descriptor(alias.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_attributes() {
        let mut visitor = GTMockVisitor::new();
        let attribute = GTAttribute::new(
            (0, 0).into(),
            GTAttributeName::new((0, 0).into(), "answer".into()).into(),
            None,
        );
        let mut alias = GTAlias {
            id: GTDefinitionId("module".into(), "Name".into()),
            span: (0, 0).into(),
            doc: None,
            attributes: vec![attribute.clone(), attribute.clone()],
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        alias.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Alias(alias.clone()),
                GTMockVisited::Attribute(attribute.clone()),
                GTMockVisited::AttributeName(attribute.name.clone()),
                GTMockVisited::Attribute(attribute.clone()),
                GTMockVisited::AttributeName(attribute.name.clone()),
                GTMockVisited::Descriptor(alias.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
