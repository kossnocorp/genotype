use genotype_parser::tree::primitive::GTPrimitive;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTPrimitive {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_primitive(self);
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
        let primitive = GTPrimitive::String;
        primitive.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::Primitive(GTPrimitive::String),]
        );
    }
}
