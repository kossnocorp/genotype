use genotype_parser::tree::array::GTArray;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTArray {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_array(self);
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
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let mut array = GTArray {
            span: (0, 0).into(),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        array.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Array(array.clone()),
                GTMockVisited::Descriptor(array.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
