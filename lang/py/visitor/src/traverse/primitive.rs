use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYPrimitive {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_primitive(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut primitive = PYPrimitive::String;
        primitive.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Primitive(PYPrimitive::String),]
        );
    }
}
