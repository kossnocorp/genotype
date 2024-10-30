use genotype_lang_py_tree::tuple::PYTuple;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYTuple {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_tuple(self);
        for descriptor in &mut self.descriptors {
            descriptor.traverse(visitor);
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
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let primitive = PYDescriptor::Primitive(PYPrimitive::String);
        let mut tuple = PYTuple {
            descriptors: vec![primitive.clone()],
        };
        tuple.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Tuple(tuple.clone()),
                PYMockVisited::Descriptor(primitive),
                PYMockVisited::Primitive(PYPrimitive::String),
            ]
        );
    }
}
