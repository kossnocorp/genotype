use crate::prelude::internal::*;

impl PYTraverse for PYPrimitive {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_primitive(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
