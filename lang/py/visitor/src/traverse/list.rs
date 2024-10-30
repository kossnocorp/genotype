use genotype_lang_py_tree::PYList;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYList {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_list(self);
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
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut list = PYList {
            descriptor: PYPrimitive::String.into(),
        };
        list.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::List(list.clone()),
                PYMockVisited::Descriptor(list.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }
}
