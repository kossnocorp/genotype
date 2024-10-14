use genotype_parser::tree::tuple::GTTuple;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTTuple {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
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
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String((0, 0).into()));
        let mut tuple = GTTuple {
            descriptors: vec![primitive.clone()],
        };
        tuple.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Tuple(tuple.clone()),
                GTMockVisited::Descriptor(primitive),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
