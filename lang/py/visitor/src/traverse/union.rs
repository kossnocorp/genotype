use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYUnion {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_union(self);
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
}
