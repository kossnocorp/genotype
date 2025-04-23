use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYNewtype {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_newtype(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        self.name.traverse(visitor);
        self.primitive.traverse(visitor);
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
        let ref_identifier = PYIdentifier("Reference".into());
        let mut alias = PYAlias {
            doc: None,
            name: PYIdentifier("Name".into()),
            descriptor: PYPrimitive::String.into(),
            references: vec![ref_identifier.clone()],
        };
        alias.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Alias(alias.clone()),
                PYMockVisited::Descriptor(alias.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
                PYMockVisited::Identifier(ref_identifier)
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = PYMockVisitor::new();
        let mut alias = PYAlias {
            doc: Some(PYDoc("Hello, world!".into())),
            name: PYIdentifier("Name".into()),
            descriptor: PYPrimitive::String.into(),
            references: vec![],
        };
        alias.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Alias(alias.clone()),
                PYMockVisited::Doc(alias.doc.clone().unwrap()),
                PYMockVisited::Descriptor(alias.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }
}
