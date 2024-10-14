use genotype_parser::tree::alias::GTAlias;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAlias {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_alias(self);
        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
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
            doc: None,
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
            doc: Some(GTDoc("Hello, world!".into())),
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
}
