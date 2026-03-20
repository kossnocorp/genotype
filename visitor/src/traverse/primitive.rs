use genotype_parser::tree::primitive::GTPrimitive;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTPrimitive {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_primitive(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_test::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let mut primitive = Gt::primitive_string();
        primitive.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::Primitive(Gt::primitive_string()),]
        );
    }
}
