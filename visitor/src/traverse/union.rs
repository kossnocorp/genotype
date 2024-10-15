use genotype_parser::tree::union::GTUnion;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTUnion {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
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
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String((0, 0).into()));
        let union = GTUnion {
            span: (0, 0).into(),
            descriptors: vec![primitive.clone()],
        };
        let mut descriptor = GTDescriptor::Union(union.clone());
        descriptor.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Union(union.clone()),
                GTMockVisited::Descriptor(primitive),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
