use crate::prelude::internal::*;

impl PYTraverse for PYKey {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_key(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut key = PYKey("name".into());
        key.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::Key(key.clone()),]);
    }
}
