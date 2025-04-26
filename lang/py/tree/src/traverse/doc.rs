use crate::prelude::internal::*;

impl PYTraverse for PYDoc {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_doc(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut doc = PYDoc("Hello, world!".into());
        doc.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::Doc(doc.clone()),]);
    }
}
